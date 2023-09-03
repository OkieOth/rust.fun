use std::sync::Arc;
use std::{thread, time};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use amqprs::{
    callbacks::{ChannelCallback, ConnectionCallback},
    channel::Channel,
    connection::{Connection, OpenConnectionArguments},
    Ack, BasicProperties, Cancel, Close, CloseChannel, Nack, Return,
};
use async_trait::async_trait;
use tokio::sync::Mutex;

use log::{debug, error, info};

type Result<T> = std::result::Result<T, amqprs::error::Error>;

#[derive(Debug, Clone, Default)]
pub struct RabbitConParams {
    pub server: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}


pub struct RabbitClient {
    con_params: RabbitConParams,
    connection: Arc<Mutex<Option<Connection>>>,
    max_reconnect_attempts: u8,
    tx_panic: Sender<String>,
    con_callback: RabbitConCallback,
}

impl RabbitClient {
    pub fn new(
        con_params: RabbitConParams,
        tx_panic: Sender<String>,
        max_reconnect_attempts: u8,
    ) -> RabbitClient {
        let (tx_con_callback, rx_con_callback): (Sender<u8>, Receiver<u8>) = mpsc::channel(1);

        let con_callback = RabbitConCallback {
            tx_con_callback: tx_con_callback,
        };

        let call_back_tx_panic = tx_panic.clone();

        let client = RabbitClient {
            con_params: con_params.clone(),
            connection: Arc::new(Mutex::new(None)),
            con_callback: con_callback.clone(),
            max_reconnect_attempts: max_reconnect_attempts,
            tx_panic: tx_panic,
        };

        listen_to_con(rx_con_callback, con_params, client.connection.clone(),con_callback.clone(), max_reconnect_attempts, call_back_tx_panic);
        return client;
    }

    pub async fn connect(&mut self) -> std::result::Result<(), String> {
        info!("connect is called :)");
        {
            let conn = self.connection.lock().await;
            if conn.is_some() {
                let o : Option<&Connection> = conn.as_ref();
                if o.unwrap().is_open() {
                    debug!("connection was already open");
                    return Ok(());
                }
            }
        }
        info!(
            "server: {}, port: {}",
            &self.con_params.server, &self.con_params.port
        );
        match reconnect(&self.con_params, self.con_callback.clone(), self.connection.clone()).await {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub async fn get_publisher(&self, id :i32) -> std::result::Result<RabbitPublisher, String> {
        let (tx_channel_callback, rx_channel_callback): (Sender<u8>, Receiver<u8>) = mpsc::channel(1);

        let channel_callback = RabbitChannelCallback {
            id: id,
            tx_channel_callback: tx_channel_callback,
        };

        let publisher =  RabbitPublisher {
            id: id,
            connection: Arc::clone(&self.connection),
            channel_callback: channel_callback,
            channel: Arc::new(Mutex::new(None)),
        };
        Ok(publisher)
    }

}

pub struct RabbitPublisher {
    id: i32,
    connection: Arc<Mutex<Option<Connection>>>,
    channel: Arc<Mutex<Option<Channel>>>,
    channel_callback: RabbitChannelCallback,
}

impl RabbitPublisher {
    pub fn say_hello(&self) {
        info!("hello, here is publisher-{}", self.id);
    }

    pub async fn open_channel(&self) -> std::result::Result<(), String> {
        debug!("channel-{}: try to get connection", self.id);
        let c = self.connection.lock().await;
        let conn = c.as_ref().unwrap();
        match conn.open_channel(None).await {
            Ok(channel) => {
                // channel
                // .register_callback(self.channel_callback.clone())
                // .await
                // .unwrap();
                let mut ch = self.channel.lock().await;
                *ch = Some(channel);
                Ok(())
            },
            Err(e) => {
                error!("error while creating channel: {}", e.to_string());
                Err("error while creating channel".to_string())
            }
        }

        // let conn = self.connection.lock().await;
        // debug!("channel-{}: got connection", self.id);
        // if conn.is_some() {
        //     let o : Option<&Connection> = conn.as_ref();
        //     let c = o.unwrap();
        //     if c.is_open() {
        //         match c.open_channel(None).await {
        //             Ok(channel) => {
        //                 // channel
        //                 // .register_callback(self.channel_callback.clone())
        //                 // .await
        //                 // .unwrap();
        //                 let mut ch = self.channel.lock().await;
        //                 *ch = Some(channel);
        //                 Ok(())
        //             },
        //             Err(e) => {
        //                 error!("error while creating channel: {}", e.to_string());
        //                 Err("error while creating channel".to_string())
        //             }
        //         }
        //     } else {
        //         return Err("connection isn't open".to_string());
        //     }
        // } else {
        //     return Err("connection isn't ready".to_string());
        // }
    }
}

fn listen_to_con(mut rx_con_callback: Receiver<u8>, con_params: RabbitConParams, con_ref: Arc<Mutex<Option<Connection>>>,
    callback_ref: RabbitConCallback, max_reconnect_attempts: u8, tx_panic: Sender<String>) {
    tokio::spawn(async move {
        while let Some(_) = rx_con_callback.recv().await {
            info!("Rabbit connection is closed");
            {
                let mut conn = con_ref.lock().await;
                *conn = None;
            }
            try_to_reconnect(&con_params, callback_ref.clone(), con_ref.clone(), max_reconnect_attempts, tx_panic.clone()).await.unwrap();
        }
    });
}

async fn try_to_reconnect(
    con_params: &RabbitConParams,
    con_callback: RabbitConCallback,
    con: Arc<Mutex<Option<Connection>>>,
    max_reconnect_attempts: u8, 
    tx_panic: Sender<String>) -> std::result::Result<(), String> {
    let mut reconnect_seconds = 1;
    let mut reconnect_attempts: u8 = 0;
    loop {
        match reconnect(con_params, con_callback.clone(), con.clone()).await {
            Ok(_) => {
                return Ok(());
            },
            Err(e) => {
                error!("error while trying to reconnect: {}", e.to_string());
                let sleep_time = time::Duration::from_secs(reconnect_seconds);
                info!("sleep for {} seconds before try to reconnect ...", reconnect_seconds);
                thread::sleep(sleep_time);
                reconnect_seconds = reconnect_seconds * 2;
                reconnect_attempts += 1;
                if reconnect_attempts > max_reconnect_attempts {
                    error!("reached maximum reconnection attempts ({}), and stop my doing", reconnect_attempts);
                    match tx_panic.send("reached maximum reconnection attempts".to_string()).await {
                        Ok(_) => {
                            debug!("sent panic request");
                        },
                        Err(e) => {
                            error!("error while sending panic request: {}", e.to_string());
                        }
                    }
                    return Err(e);
                }
            }
        }
    }

}


async fn reconnect(
    con_params: &RabbitConParams,
    con_callback: RabbitConCallback,
    con: Arc<Mutex<Option<Connection>>>,
) -> std::result::Result<(), String> {
    match Connection::open(&OpenConnectionArguments::new(
        &con_params.server,
        con_params.port,
        &con_params.user,
        &con_params.password,
    ))
    .await
    {
        Ok(connection) => {
            info!("connection established :)");
            connection
                .register_callback(con_callback.clone())
                .await
                .unwrap();
            info!("???: {}", connection.is_open());
            let mut conn = con.lock().await;
            *conn = Some(connection);
            Ok(())
        }
        Err(e) => {
            error!("connection failure :(");
            Err(e.to_string())
        }
    }
}

#[derive(Debug, Clone)]
struct RabbitConCallback {
    tx_con_callback: Sender<u8>,
}


#[async_trait]
impl ConnectionCallback for RabbitConCallback {
    async fn close(&mut self, _: &Connection, _: Close) -> Result<()> {
        info!("connection was closed");

        if let Err(e) = self.tx_con_callback.send(1).await {
            error!(
                "error while notify about closed connection: {}",
                e.to_string()
            )
        }
        Ok(())
    }

    async fn blocked(&mut self, _: &Connection, _: String) {
        debug!("connection is blocked")
    }
    async fn unblocked(&mut self, _: &Connection) {
        debug!("connection is unblocked")
    }
}

#[derive(Debug, Clone)]
struct RabbitChannelCallback {
    id: i32,
    tx_channel_callback: Sender<u8>,
}

#[async_trait]
impl ChannelCallback for RabbitChannelCallback {
    async fn close(&mut self, channel: &Channel, close: CloseChannel) -> Result<()> {
        info!("channel-{} was closed", self.id);

        if let Err(e) = self.tx_channel_callback.send(1).await {
            error!(
                "error while notify about closed channel: {}",
                e.to_string()
            )
        }
        Ok(())
    }
    async fn cancel(&mut self, channel: &Channel, cancel: Cancel) -> Result<()> {
        Ok(())
    }
    async fn flow(&mut self, channel: &Channel, active: bool) -> Result<bool> {
        Ok(true)
    }
    async fn publish_ack(&mut self, channel: &Channel, ack: Ack) {}
    async fn publish_nack(&mut self, channel: &Channel, nack: Nack) {}
    async fn publish_return(
        &mut self,
        channel: &Channel,
        ret: Return,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
    }
}

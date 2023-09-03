
use std::error::Error;
use std::fmt;
use std::sync::{Arc};
use std::sync::mpsc::{Sender};

use amqprs::connection;
use tokio::sync::{Mutex};
use amqprs::{
    callbacks::{ChannelCallback, ConnectionCallback},
    channel::Channel,
    channel::{
        BasicConsumeArguments, BasicPublishArguments, QueueBindArguments, QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    Ack, BasicProperties, Cancel, Close, CloseChannel, Nack, Return,
};
use std::{thread, time};
use async_trait::async_trait;

use log::{error, info, debug};

type Result<T> = std::result::Result<T, amqprs::error::Error>;

#[derive(Debug)]

#[derive(Clone, Default)]
pub struct RabbitConParams {
    pub server: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

struct RabbitCon {
    con_params: RabbitConParams,
    connection: Option<Connection>,
    channel_publishing: Option<Channel>,
    channel_subscribing: Option<Channel>,
    tx_panic: Sender<String>,
    max_reconnect_attempts: u8,
}

pub struct RabbitClient {
    con: Arc<Mutex<RabbitCon>>
}

pub struct Publisher {
    channel_publishing: Option<Channel>,
}

impl RabbitClient {
    pub fn new(con_params: RabbitConParams, tx_panic: Sender<String>, max_reconnect_attempts: u8) -> RabbitClient {
        let rabbit_con = RabbitCon {
            con_params: con_params,
            connection: None,
            channel_publishing: None,
            channel_subscribing: None,
            tx_panic: tx_panic,
            max_reconnect_attempts: max_reconnect_attempts,
        };
        RabbitClient {
            con: Arc::new(Mutex::new(rabbit_con)),
        }
    }

    pub async fn connect(&mut self) -> std::result::Result<(),String> {
        info!("connect is called :)");
        let mut rabbit_con = self.con.lock().await;
        if rabbit_con.connection.is_some() {
            let con = rabbit_con.connection.as_ref().unwrap();
            if con.is_open() {
                debug!("connection was already open");
                return Ok(());
            }
        }
        info!("server: {}, port: {}", &rabbit_con.con_params.server, &rabbit_con.con_params.port);
        match Connection::open(&OpenConnectionArguments::new(
            &rabbit_con.con_params.server,
            rabbit_con.con_params.port,
            &rabbit_con.con_params.user,
            &rabbit_con.con_params.password,
        ))
        .await {
            Ok(connection) => {
                info!("connection established :)");
                rabbit_con.connection = Some(connection);
                rabbit_con.connection.as_ref()
                    .unwrap()
                    .register_callback(RabbitClientConCallback {con: self.con.clone()})
                    .await
                    .unwrap();
                Ok(())
            },
            Err(e) => {
                error!("connection failure :(");
                Err(e.to_string())
            },
        }
    }

    pub async fn get_publisher(&self) -> Publisher {
        match self.prepare_publish().await {
            Ok(()) => {
                debug!("publishing channel is ready")
            },
            Err(e) => {
                // Handle channel creation
            },
        };
        let rabbit_con = self.con.lock().await;
        let channel = rabbit_con.channel_publishing.as_ref().unwrap();
        Publisher {
            channel_publishing: Some(channel.clone()),
        }
    }

    async fn prepare_publish(&self) -> std::result::Result<(),String> {
        let mut rabbit_con = self.con.lock().await;
        if rabbit_con.connection.is_none() {
            return Err("connection isn't available".to_string());
        }
        if rabbit_con.channel_publishing.is_none() {
            // create the needed channel
            let channel = rabbit_con.connection.as_ref().unwrap().open_channel(None).await.unwrap();
            rabbit_con.channel_publishing = Some(channel);
        }
        Ok(())
    }
    
}

impl Publisher {
    pub async fn publish(&self, exchange: String, routing_key: String, content: Vec<u8>) -> std::result::Result<(), String> {
        let args = BasicPublishArguments::new(&exchange, &routing_key);
        match self.channel_publishing.as_ref().unwrap().basic_publish(BasicProperties::default(), content, args).await {
            Ok(()) => {
                Ok(())
            },
            Err(e) => Err(e.to_string())
        }
    }
}


#[async_trait]
impl ChannelCallback for Publisher {
    async fn close(&mut self, channel: &Channel, close: CloseChannel) -> Result<()> {
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



struct RabbitClientConCallback {
    con: Arc<Mutex<RabbitCon>>
}

#[async_trait]
impl ConnectionCallback for RabbitClientConCallback {
    async fn close(&mut self, connection: &Connection, close: Close) -> Result<()> {
        debug!("connection was closed");
        let mut reconnect_seconds = 1;
        let mut reconnect_attempts: u8 = 0;
        let mut rabbit_con = self.con.lock().await;
        rabbit_con.connection = None;
        loop {
            match Connection::open(&OpenConnectionArguments::new(
                &rabbit_con.con_params.server,
                rabbit_con.con_params.port,
                &rabbit_con.con_params.user,
                &rabbit_con.con_params.password,
                ))
            .await {
                Ok(connection) => {
                    info!("connection established :)");
                    rabbit_con.connection = Some(connection);
                    rabbit_con.connection.as_ref()
                        .unwrap()
                        .register_callback(RabbitClientConCallback {con: self.con.clone()})
                        .await
                        .unwrap();
                    break;
                },
                Err(e) => {
                    error!("error while trying to reconnect: {}", e.to_string());
                    let sleep_time = time::Duration::from_secs(reconnect_seconds);
                    info!("sleep for {} seconds before try to reconnect ...", reconnect_seconds);
                    thread::sleep(sleep_time);
                    reconnect_seconds = reconnect_seconds * 2;
                    reconnect_attempts += 1;
                    if reconnect_attempts > rabbit_con.max_reconnect_attempts {
                        error!("reached maximum reconnection attempts, and stop my doing");
                        rabbit_con.tx_panic.send("reached maximum reconnection attempts".to_string());
                        return Err(e);
                        //break;
                    }
                },
            };
        }
        Ok(())
    }

    async fn blocked(&mut self, connection: &Connection, reason: String) {
        debug!("connection is blocked")
    }
    async fn unblocked(&mut self, connection: &Connection) {
        debug!("connection is unblocked")
    }
}
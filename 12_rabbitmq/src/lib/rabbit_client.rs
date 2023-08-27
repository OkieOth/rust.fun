use amqprs::{
    callbacks::{ChannelCallback, ConnectionCallback},
    channel::Channel,
    connection::{Connection, OpenConnectionArguments},
    Ack, BasicProperties, Cancel, Close, CloseChannel, Nack, Return,
};
use async_trait::async_trait;

use log::{error, info, debug};

type Result<T> = std::result::Result<T, amqprs::error::Error>;


#[derive(Clone)]
pub struct RabbitConParams {
    pub server: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}


pub struct RabbitClient {
    con_params: RabbitConParams,
    connection: Option<Connection>,
    channel_publishing: Option<Channel>,
    channel_subscribing: Option<Channel>,
}

impl RabbitClient {
    pub fn new(con_params: RabbitConParams) -> RabbitClient {
        RabbitClient {
            con_params: con_params.clone(),
            connection: None,
            channel_publishing: None,
            channel_subscribing: None,
        }
    }

    pub async fn connect(&mut self) -> std::result::Result<(),String> {
        info!("connect is called :)");
        match Connection::open(&OpenConnectionArguments::new(
            &self.con_params.server,
            self.con_params.port,
            &self.con_params.user,
            &self.con_params.password,
        ))
        .await {
            Ok(connection) => {
                info!("connection established :)");
                self.connection = Some(connection);
                self.connection.as_ref()
                    .unwrap()
                    .register_callback(RabbitClientConCallback {})
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

    pub async fn publish(&self, exchange: String, routing_key: String) -> Result<()> {
        Ok(())
    }
}

struct RabbitClientConCallback {}

#[async_trait]
impl ConnectionCallback for RabbitClientConCallback {
    async fn close(&mut self, connection: &Connection, close: Close) -> Result<()> {
        debug!("connection was closed");
        Ok(())
    }

    async fn blocked(&mut self, connection: &Connection, reason: String) {
        debug!("connection is blocked")
    }
    async fn unblocked(&mut self, connection: &Connection) {
        debug!("connection is unblocked")
    }
}
use amqprs::{
    connection::{Connection, OpenConnectionArguments}
};

use log::{error, info, debug};

pub struct RabbitConnection {
    connection: Option<Connection>,
}

impl RabbitConnection {
    pub fn new() -> RabbitConnection {
        RabbitConnection { connection: None }
    }

    pub async fn connect(&mut self) -> Result<(),&'static str> {
        info!("connect is called :)");
        match Connection::open(&OpenConnectionArguments::new(
            "localhost",
            5673,
            "guest",
            "guest",
        ))
        .await {
            Ok(connection) => {
                info!("connection established :)");
                self.connection = Some(connection);
                Ok(())
            },
            Err(e) => {
                error!("connection failure :(");
                Err("This is a test")
            },
        }
    }

    pub async fn publish(&mut self, content: Vec<u8>) -> Result<(),&'static str> {
        Ok(())
    }

}



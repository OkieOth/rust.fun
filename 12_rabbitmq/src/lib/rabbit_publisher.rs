use amqprs::{
    connection::{Connection, OpenConnectionArguments}
};

use log::{error, info, debug};

#[derive(Default)]
pub struct RabbitPublisherParams {
    exchange: Option<String>,

    /// Routing-key to use
    routing_key: Option<String>,

    /// Connection string
    // default_value = "amqp://localhost:5672")]
    connection: Option<String>,

    /// User name to authenticate
    user: Option<String>,

    /// Password to authenticate
    password: Option<String>,
}

pub struct RabbitPublisher {
    connection: Option<Connection>,
    params: RabbitPublisherParams,
}

pub struct PublisherError {
}


impl RabbitPublisher {
    pub fn new(params: RabbitPublisherParams) -> RabbitPublisher {
        RabbitPublisher {
            connection: None,
            params: params,
        }
    }

    pub fn publish(self, content: Vec<u8>) -> Result<(), PublisherError> {
        Ok(())
    }

    pub fn publish_to(self, content: Vec<u8>, exchange: String, routing_key: String) -> Result<(), PublisherError> {
        Ok(())
    }
}


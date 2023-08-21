# Some commands to investigate rabbitmq

* https://www.rabbitmq.com/management-cli.html
* https://www.rabbitmq.com/rabbitmq-diagnostics.8.html
* https://www.rabbitmq.com/rabbitmqctl.8.html
* https://www.rabbitmq.com/rabbitmq-queues.8.html

```bash
## rabbitmq-diagnostics
# enters the docker compose based rabbitmq and ..
# .. queries the existing exchanges
docker exec `docker ps | grep 12_rabbit | awk '{print $1}'` rabbitmq-diagnostics list_exchanges

# create a dead-letter queue

# serialize data from a queue

## rabbitmqadim
docker exec `docker ps | grep 12_rabbit | awk '{print $1}'` rabbitmqadmin list exchanges

rabbitmqadmin remove exchange name=eikos.test type=topic
rabbitmqadmin declare binding source="eiko.test" \ 
    destination_type="queue" \
    destination="eikos_queue" \
    routing_key="*"
```
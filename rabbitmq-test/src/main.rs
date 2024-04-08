use amqprs::{callbacks::{DefaultChannelCallback, DefaultConnectionCallback}, channel::{
    Channel,
    BasicConsumeArguments,
    QueueBindArguments,
    QueueDeclareArguments,
}, connection::{Connection, OpenConnectionArguments}, consumer::DefaultConsumer, BasicProperties, Deliver};
use amqprs::channel::{BasicAckArguments, BasicPublishArguments};
use amqprs::consumer::AsyncConsumer;
use async_trait::async_trait;
use tokio::sync::Notify;

use tokio::time;

// use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    println!("Hello, world!");

    // tracing_subscriber::registry()
    //     .with(fmt::layer)
    //     .with(EnvFilter::from_default_env())
    //     .try_init()
    //     .ok();

    let connection = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "guest",
        "guest",
    )).await.unwrap();

    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();    

    let channel = connection.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();   

    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::durable_client_named(
            "amqprs.sample.test_queue",
        ))
        .await
        .unwrap()
        .unwrap();

    // bind queue to exchange
    let routing_key = "amqprs.sample";
    let exchange_name = "amq.topic";
    channel.queue_bind(QueueBindArguments::new(
        &queue_name, 
        exchange_name, 
        routing_key
    ))
    .await
    .unwrap();


    let args = BasicConsumeArguments::new(&queue_name, "example_consumer");
    //channel.basic_consume(DefaultConsumer::new(args.no_ack), args).await.unwrap();
    channel.basic_consume(MyConsumer, args).await.unwrap();

    // basic_publish
    let payload = b"Hello, world!";
    let args2 = BasicPublishArguments::new(exchange_name, routing_key);
    channel.basic_publish(BasicProperties::default(), payload.to_vec(), args2).await.unwrap();

    // time::sleep(time::Duration::from_secs(10)).await;
    // channel.close().await.unwrap();
    // connection.close().await.unwrap();

    println!("Press ctlr+c key to exit...");
    let guard = Notify::new();
    guard.notified().await;
}

struct MyConsumer;

#[async_trait]
impl AsyncConsumer for MyConsumer {
    async fn consume(&mut self, channel: &Channel, deliver: Deliver, basic_properties: BasicProperties, content: Vec<u8>) {
        let message = String::from_utf8(content).unwrap();
        println!("receive message: {}", message);

        let args = BasicAckArguments::new(deliver.delivery_tag(), false);
        channel.basic_ack(args).await.unwrap();
    }
}

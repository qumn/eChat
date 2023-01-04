#![allow(dead_code)]

use std::collections::HashMap;

use tokio::sync::{
    broadcast::{self},
    mpsc, oneshot,
};
pub mod modles;
pub mod utils;
pub mod err;

#[derive(Debug)]
pub enum Error {
    BusTermination,
    NoSubscriber,
}

const QUEUE_SIZE: usize = 100;
type Subscribers<T> = HashMap<String, broadcast::Sender<T>>;

pub enum Event<T> {
    NewMessage {
        key: String,
        val: T,
        sender: oneshot::Sender<Result<(), Error>>,
    },
    NewSubscriber {
        key: String,
        sender: oneshot::Sender<broadcast::Receiver<T>>,
    },
}
// evenBus.publish("key", value);
// let receiver = evenBus.subscriber("key"); // multiple subscriber
// while Some(value) = receiver.recv() {}
// use broadcast implement publish/subscribe pattern
// 多个 生产者, 多个消费者. 一个message 必须被所有的消费者消费
struct MessageBus<T>
where
    T: Clone + Send + Sync + 'static,
{
    sender: mpsc::Sender<Event<T>>,
}

impl<T> MessageBus<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        let (bus, message_manage) = MessageBusManage::new();
        message_manage.run();
        bus       
    }
    pub async fn publish(&self, key: String, val: T) -> Result<(), Error> {
        let (sender, receiver) = oneshot::channel();
        let event = Event::NewMessage { key, val, sender };
        self.sender
            .send(event)
            .await
            .map_err(|_| Error::BusTermination)?;
        receiver.await.unwrap()
    }

    pub async fn subscribe(&self, key: String) -> Result<broadcast::Receiver<T>, Error> {
        let (sender, receiver) = oneshot::channel();
        let event = Event::NewSubscriber { key, sender };
        self.sender
            .send(event)
            .await
            .map_err(|_| Error::BusTermination)?;
        receiver.await.map_err(|_| Error::NoSubscriber)
    }
}

struct MessageBusManage<T>
where
    T: Clone + Send + Sync + 'static,
{
    subscribers: Subscribers<T>,
    receiver: mpsc::Receiver<Event<T>>,
}

impl<T> MessageBusManage<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new() -> (MessageBus<T>, MessageBusManage<T>) {
        let (sender, receiver) = mpsc::channel(QUEUE_SIZE);
        (
            MessageBus { sender },
            MessageBusManage {
                subscribers: HashMap::new(),
                receiver,
            },
        )
    }

    pub fn run(mut self) {
        tokio::spawn(async move {
            while let Some(event) = self.receiver.recv().await {
                match event {
                    Event::NewMessage { key, val, sender } => {
                        let result = self
                            .subscribers
                            .get(&key)
                            .ok_or(Error::NoSubscriber)
                            .and_then(|sd| {
                                sd.send(val).map(|_| ()).map_err(|_| Error::NoSubscriber)
                            });
                        // ignore the situation which caller no longer care the result
                        let _ = sender.send(result);
                    }
                    Event::NewSubscriber { key, sender } => {
                        let tx = self.subscribers.entry(key).or_insert_with(|| {
                            let (tx, _) = broadcast::channel(QUEUE_SIZE);
                            tx
                        });
                        // ignore the situation which caller no longer care the result
                        let _ = sender.send(tx.subscribe());
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use tokio::time::sleep;

    use super::*;

    #[tokio::test]
    async fn publish_subscribe_should_work(){
        let bus: MessageBus<String> = MessageBus::new();

        let mut receiver = bus.subscribe("alice".to_string()).await.unwrap();
        let join_handle = tokio::spawn(async move {
            while let Ok(val) = receiver.recv().await {
                assert_eq!(val, "one");
                assert_eq!(val, "two");
            }
        });
        // it is must to wait for the subscriber to be ready
        join_handle.await.unwrap();

        bus.publish(String::from("alice"), String::from("one")).await.unwrap();
        bus.publish(String::from("alice"), String::from("two")).await.unwrap();
    }

    #[tokio::test]
    async fn multiple_subscribe_should_work(){
        let bus: MessageBus<String> = MessageBus::new();

        let mut receiver = bus.subscribe("alice".to_string()).await.unwrap();
        let subscriber1 = tokio::spawn(async move {
            while let Ok(val) = receiver.recv().await {
                assert_eq!(val, "one");
                assert_eq!(val, "two");
            }
        });

        let mut receiver = bus.subscribe("alice".to_string()).await.unwrap();
        let subscriber2 = tokio::spawn(async move {
            while let Ok(val) = receiver.recv().await {
                assert_eq!(val, "one");
                assert_eq!(val, "two");
            }
        });
        // it is must to wait for the subscriber to be ready
        subscriber1.await.unwrap();
        subscriber2.await.unwrap();

        bus.publish(String::from("alice"), String::from("one")).await.unwrap();
        bus.publish(String::from("alice"), String::from("two")).await.unwrap();

    }
    #[tokio::test]
    async fn how_sleep_work(){
        sleep(Duration::from_secs(3)).await;
    }
}

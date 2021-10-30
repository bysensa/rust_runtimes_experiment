pub mod dog_fact;

use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::spawn;
use kompact::prelude::*;
use kompact::executors;
use kompact::executors::{CanExecute, Executor, FuturesExecutor, JoinHandle};
use tokio::task::spawn_blocking;
use client::DogFact;
use tide::TideServer;
use dog_fact::{DogFactComponent, DogFactRequest, DogFactResponse};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tokio::spawn(async move {
        let tide_server = TideServer::new();
        dbg!(&tide_server);
        tide_server.listen("127.0.0.1:8080").await
    });


    let mut config = KompactConfig::default();
    let system = config.build().expect("system");

    let dog_fact = system.create(DogFactComponent::new);
    let dog_fact_ref = dog_fact.actor_ref().hold().expect("live");
    system.start(&dog_fact);

    let answer = dog_fact_ref.ask(Ask::of(DogFactRequest));
    let handle = system.spawn(async move {
        let result = answer.await.unwrap();
        dbg!(result.0);
    });

    system.await_termination();
    Ok(())
}

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
    let component = system.create(HelloWorldComponent::new);
    let dog_fact = system.create(DogFactComponent::new);
    let dog_fact_ref = dog_fact.actor_ref().hold().expect("live");
    let answer = dog_fact_ref.ask(Ask::of(DogFactRequest));
    let handle = system.spawn(async move {
        let result = answer.await.unwrap();
        dbg!(result.0);
    });

    system.start(&component);
    system.start(&dog_fact);
    system.await_termination();
    Ok(())
}


#[derive(ComponentDefinition)]
struct HelloWorldComponent {
    ctx: ComponentContext<Self>
}
impl HelloWorldComponent {
    pub fn new() -> HelloWorldComponent {
        HelloWorldComponent {
            ctx: ComponentContext::uninitialised()
        }
    }
}

enum CatFactMessage {
    CatFactRequest,
    CatFactResponse(String)
}

impl Actor for HelloWorldComponent {
    type Message = ();

    fn receive_local(&mut self, msg: Self::Message) -> Handled {
        todo!()
    }

    fn receive_network(&mut self, msg: NetMessage) -> Handled {
        Handled::Ok
    }
}

impl ComponentLifecycle for HelloWorldComponent {
    fn on_start(&mut self) -> Handled {
        info!(self.ctx.log(), "Hello World!");
        // self.ctx().system().shutdown_async();
        Handled::Ok
    }
}


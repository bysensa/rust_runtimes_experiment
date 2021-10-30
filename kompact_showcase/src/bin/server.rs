

use std::sync::Arc;
use kompact::component::{ComponentLifecycle, ComponentLogging, Handled};
use kompact::{JoinHandle, KompactLogger};
use kompact::prelude::*;
use tokio::runtime;
use client::DogFact;
use tide_server::TideServer;
use super::dog_fact::{DogFactComponent, DogFactRequest};

type Server = Arc<Box<TideServer>>;

#[derive(ComponentDefinition, Actor)]
pub struct ServerComponent {
    ctx: ComponentContext<Self>,
    handle: Option<JoinHandle<()>>,
}

impl ServerComponent {
    pub fn new() -> ServerComponent {
        ServerComponent {
            ctx: ComponentContext::uninitialised(),
            handle: None,
        }
    }

}


impl ComponentLifecycle for ServerComponent {
    fn on_start(&mut self) -> Handled where Self: 'static {
        let dog_fact = self.ctx.system().create(DogFactComponent::new);
        let dog_fact_ref = dog_fact.actor_ref().hold().unwrap();
        self.handle = Some(self.ctx.system().spawn(async move {
            let rt = runtime::Builder::new_multi_thread().enable_all().build().unwrap();
            rt.block_on(async move {
                //dog_fact_ref.ask(Ask::of(DogFactRequest));
                let server = TideServer::new();
                server.listen("127.0.0.1:8080").await;
            });
        }));
        dbg!(&self.handle);
        Handled::Ok
    }


}

fn main() {}
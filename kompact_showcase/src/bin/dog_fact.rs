use client::DogFact;
use kompact::prelude::*;
use tokio;

#[derive(ComponentDefinition)]
pub struct DogFactComponent {
    ctx: ComponentContext<Self>,
}
impl DogFactComponent {
    pub fn new() -> DogFactComponent {
        DogFactComponent {
            ctx: ComponentContext::uninitialised(),
        }
    }
}

#[derive(Debug)]
pub struct DogFactRequest;
#[derive(Debug)]
pub struct DogFactResponse(String);

impl Actor for DogFactComponent {
    type Message = Ask<DogFactRequest, DogFactResponse>;

    fn receive_local(&mut self, msg: Self::Message) -> Handled {
        self.spawn_local(move |async_self| async move {
            let res = tokio::spawn(DogFact::random()).await.unwrap().unwrap();
            debug!(async_self.log(), "Got fact: {}", res);
            msg.reply(DogFactResponse(res)).expect("reply");
            Handled::Ok
        });
        Handled::Ok
    }

    fn receive_network(&mut self, msg: NetMessage) -> Handled {
        Handled::Ok
    }
}

impl ComponentLifecycle for DogFactComponent {
    fn on_start(&mut self) -> Handled {
        info!(self.ctx.log(), "Hello World!");
        // self.ctx().system().shutdown_async();
        Handled::Ok
    }
}

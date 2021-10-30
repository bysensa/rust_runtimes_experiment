use std::sync::Arc;
use std::time::Instant;
use client::DogFact;
use kompact::prelude::*;
use tokio::runtime;

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
pub struct DogFactResponse(pub String);

impl Actor for DogFactComponent {
    type Message = Ask<DogFactRequest, DogFactResponse>;

    fn receive_local(&mut self, msg: Self::Message) -> Handled {
        self.spawn_local(move |async_self| async move {
            let now = Instant::now();
            let rt = runtime::Builder::new_current_thread().enable_all().build().unwrap();
            let safe_async_self = Arc::new(async_self);
            let cloned_async_self = safe_async_self.clone();
            let res = rt.block_on(async move {
                debug!(&cloned_async_self.log(), "From Tokio runtime");
                DogFact::random().await
            }).unwrap();
            let elapsed = now.elapsed();
            debug!(safe_async_self.log(), "Time to perform logic: {}", elapsed.as_millis());
            dbg!(&res);
            msg.reply(DogFactResponse(res)).expect("reply");
            // safe_async_self.ctx.system().shutdown_async();
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

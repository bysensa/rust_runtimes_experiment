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

#[derive(Clone, Debug)]
struct TokioExecutor {
    active: Arc<AtomicBool>,
}

impl TokioExecutor {
    fn new(cpus: usize) -> TokioExecutor {
        TokioExecutor {active: Arc::new(AtomicBool::new(true)), }
    }
}

impl Default for TokioExecutor {
    fn default() -> Self {
        TokioExecutor::new(num_cpus::get())
    }
}

impl Executor for TokioExecutor {
    fn shutdown_async(&self) {
        if self.active.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst).unwrap() {
            println!("Shutting down executor.");
        } else {
            println!("Executor was already shut down!");
        }
    }

    fn shutdown_borrowed(&self) -> Result<(), String> {
        if self.active.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst).unwrap() {
            println!("Waiting for pool to shut down.");
            ///self.pool.join();
            println!("Pool was shut down.");
            Result::Ok(())
        } else {
            Result::Err(String::from("Pool was already shut down!"))
        }
    }
}

impl CanExecute for TokioExecutor {
    fn execute_job(&self, job: Box<dyn FnOnce() + Send + 'static>) {
        if self.active.load(Ordering::SeqCst) {
            tokio::spawn(async move{
                tokio::task::spawn_blocking(job);
            });
        } else {
            println!("Ignoring job as pool is shutting down.");
        }
    }
}


impl FuturesExecutor for TokioExecutor {
    fn spawn<R: Send + 'static>(&self, future: impl Future<Output=R> + 'static + Send) -> JoinHandle<R> {
        let (task, handle) = async_task::spawn(future, move |task| {
            tokio::task::spawn_blocking(|| task.run());
        });
        task.schedule();
        handle
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tokio::spawn(async move {
        let tide_server = TideServer::new();
        dbg!(&tide_server);
        tide_server.listen("127.0.0.1:8080").await
    });


    let mut config = KompactConfig::default();
    // config.executor(TokioExecutor::new);
    let system = config.build().expect("system");
    let component = system.create(HelloWorldComponent::new);
    let dog_fact = system.create(DogFactComponent::new);
    let dog_fact_ref = dog_fact.actor_ref().hold().expect("live");
    dog_fact_ref.ask(Ask::of(DogFactRequest));
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


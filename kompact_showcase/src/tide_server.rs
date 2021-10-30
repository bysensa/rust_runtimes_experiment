pub mod tide_server {

    use std::sync::Arc;
    use kompact::component::{ComponentLifecycle, ComponentLogging, Handled};
    use kompact::{JoinHandle, KompactLogger};
    use kompact::prelude::*;
    use std::io;
    use std::net::{SocketAddr};
    use kompact::prelude::{ActorRefStrong, Ask};
    use tide::{Request, Server};
    use tide::prelude::*;
    use tokio::runtime;

    #[derive(ComponentDefinition)]
    pub struct TideServerComponent {
        ctx: ComponentContext<Self>,
        handle: Option<JoinHandle<()>>,
    }

    impl TideServerComponent {
        pub fn new() -> TideServerComponent {
            TideServerComponent {
                ctx: ComponentContext::uninitialised(),
                handle: None,
            }
        }

    }

    impl Actor for TideServerComponent {
        type Message = Ask<TideReqData, TideResData>;

        fn receive_local(&mut self, msg: Self::Message) -> Handled {
            dbg!(&msg.request());

            msg.reply(TideResData);
            Handled::Ok
        }

        fn receive_network(&mut self, msg: NetMessage) -> Handled {
            unimplemented!("Network messages not supported")
        }
    }


    impl ComponentLifecycle for TideServerComponent {
        fn on_start(&mut self) -> Handled where Self: 'static {
            let self_ref = self.actor_ref().hold().unwrap();
            self.handle = Some(self.ctx.system().spawn(async move {
                let rt = runtime::Builder::new_multi_thread().enable_all().build().unwrap();
                rt.block_on(async move {
                    let server = TideServer::new(TideServerState{
                        actor_ref: self_ref
                    });
                    server.listen("127.0.0.1:8080").await;
                });
            }));
            dbg!(&self.handle);
            Handled::Ok
        }


    }


    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }
    }

    #[derive(Debug)]
    pub struct TideReqData;
    #[derive(Debug)]
    pub struct TideResData;

    #[derive(Debug, Clone)]
    pub struct TideServerState {
        pub actor_ref: ActorRefStrong<Ask<TideReqData, TideResData>>
    }

    #[derive(Debug, Deserialize)]
    struct Animal {
        name: String,
        legs: u8,
    }

    #[derive(Debug)]
    pub struct TideServer {
        server: Server<TideServerState>,
    }

    impl TideServer {
        pub fn new(state: TideServerState) -> TideServer {
            let mut app = tide::with_state(state);
            app.at("/").get(order_shoes);
            TideServer { server: app }
        }

        pub async fn listen(self, addr: &str ) -> io::Result<()> {
            self.server.listen(addr).await
        }
    }


    async fn order_shoes(mut req: Request<TideServerState>) -> tide::Result {
        let res = req.state().actor_ref.ask(Ask::of(TideReqData)).await;
        if let Ok(data) = res {
            dbg!(data);
        }
        Ok(format!("Hello, {}! I've put in an order for {} shoes", "somebody", 2).into())
    }


}

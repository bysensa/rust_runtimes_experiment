pub mod warp_server {

    use std::sync::Arc;
    use kompact::component::{ComponentLifecycle, ComponentLogging, Handled};
    use kompact::{JoinHandle, KompactLogger};
    use kompact::prelude::*;
    use std::convert::Infallible;
    use std::net::{SocketAddr, ToSocketAddrs};
    use kompact::prelude::{ActorRefStrong, Ask};
    use tokio::runtime;
    use warp::{Filter, Reply, Server,};


    #[derive(ComponentDefinition)]
    pub struct WarpServerComponent {
        ctx: ComponentContext<Self>,
        handle: Option<JoinHandle<()>>,
    }

    impl WarpServerComponent {
        pub fn new() -> WarpServerComponent {
            WarpServerComponent {
                ctx: ComponentContext::uninitialised(),
                handle: None,
            }
        }

    }

    impl Actor for WarpServerComponent {
        type Message = Ask<WarpReqData, WarpResData>;

        fn receive_local(&mut self, msg: Self::Message) -> Handled {
            dbg!(&msg.request());

            msg.reply(WarpResData);
            Handled::Ok
        }

        fn receive_network(&mut self, msg: NetMessage) -> Handled {
            unimplemented!("Network messages not supported")
        }
    }


    impl ComponentLifecycle for WarpServerComponent {
        fn on_start(&mut self) -> Handled where Self: 'static {
            let self_ref = self.actor_ref().hold().unwrap();
            self.handle = Some(self.ctx.system().spawn(async move {
                let rt = runtime::Builder::new_multi_thread().enable_all().build().unwrap();
                rt.block_on(async move {
                    let server = WarpServer::new(WarpServerState{
                        actor_ref: self_ref
                    });
                    server.listen(([127, 0, 0, 1], 8081)).await;
                });
            }));
            dbg!(&self.handle);
            Handled::Ok
        }


    }


    #[derive(Debug)]
    pub struct WarpReqData;

    #[derive(Debug)]
    pub struct WarpResData;

    #[derive(Debug, Clone)]
    pub struct WarpServerState {
        pub actor_ref: ActorRefStrong<Ask<WarpReqData, WarpResData>>,
    }

    pub struct WarpServer{
        state: WarpServerState,
    }

    impl WarpServer {

        pub fn new(state: WarpServerState) -> WarpServer {
            WarpServer { state }
        }

        pub async fn listen(self, addr: impl Into<SocketAddr>) {
            let route = warp::path::end().and(warp::get()).and(with_db(self.state)).and_then(reply);
            warp::serve(route).run(addr).await;
        }
    }

    fn with_db(state: WarpServerState) -> impl Filter<Extract=(WarpServerState, ), Error=std::convert::Infallible> + Clone {
        warp::any().map(move || state.clone())
    }

    pub async fn reply(state: WarpServerState) -> Result<impl warp::Reply, Infallible> {
        if let Ok(data) = state.actor_ref.ask(Ask::of(WarpReqData)).await {
            dbg!(&data);
        }
        Ok(format!("Hello, {}! I've put in an order for {} shoes", "somebody", 2))
    }


}



pub mod warp_server {
    use std::borrow::Borrow;
    use std::sync::Arc;
    use std::convert::Infallible;
    use std::net::{SocketAddr, ToSocketAddrs};
    use bastion::prelude::*;
    use tokio::runtime;
    use warp::{Filter, Reply, Server,};


    #[derive(Debug)]
    pub struct WarpReqData;

    #[derive(Debug)]
    pub struct WarpResData;

    #[derive(Debug, Clone)]
    pub struct WarpServerState {
        pub ctx: Arc<BastionContext>,
        pub associated_child: ChildrenRef,
    }

    pub struct WarpServer{
        state: WarpServerState,
    }

    impl WarpServer {

        // создаем сервер со связанным стейтом
        pub fn new(state: WarpServerState) -> WarpServer {
            WarpServer { state }
        }

        // запускаем серрвер
        pub async fn listen(self, addr: impl Into<SocketAddr>) {
            let route = warp::path::end().and(warp::get()).and(with_db(self.state)).and_then(reply);
            warp::serve(route).run(addr).await;
        }
    }

    fn with_db(state: WarpServerState) -> impl Filter<Extract=(WarpServerState, ), Error=std::convert::Infallible> + Clone {
        warp::any().map(move || state.clone())
    }

    pub async fn reply(state: WarpServerState) -> Result<impl warp::Reply, Infallible> {
        // получаем из стейта адресс группу чайлдов связанных с сервером и выбираем одного
        let child_ref = state.associated_child.elems().first().expect("child present");
        // отправляем сообщение чаилду
        let answer = state.ctx.ask(&child_ref.addr(), "".to_string()).expect("should ask");

        // ожидаем ответ от чаилда и отправляем его в ответ на запрос
        msg! { answer.await.unwrap(),
            ans: String => {
                Ok("receive answer")
            };
            unknown:_ => {
                Ok("Receive another answer")
            };
        }
    }


}

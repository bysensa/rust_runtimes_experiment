use std::sync::Arc;
use bastion::prelude::*;
use tokio::runtime;
use bastion_showcase::warp_server::warp_server::{WarpServer, WarpServerState};

fn main() {
    Bastion::init();

    // создаем ветку чаилдов для сервера
    Bastion::children(|children| {
        children.with_name("warp").with_exec(|ctx| {
            async move {
                // создаем чаилда для обработки запросов сервера
                let server_child = Bastion::children(|children| {
                    children.with_name("warp_consumer").with_exec(|ctx|{
                        async move {
                            loop {
                                msg! {
                        ctx.recv().await?,
                        message: String =!> {
                            answer!(ctx, "you receive answer").expect("Couldn't send an answer.");
                        };
                        _: _ => ();
                    }
                            }
                        }
                    })
                }).expect("server child should be present");

                // создаем блокирующую таску для сервера
                let warp_task = blocking! {
                    // создаем tokio runtime для сервера
                    let rt = runtime::Builder::new_multi_thread().enable_all().build().unwrap();
                    // запускаем сервер в созданном Tokio Runtime.
                    rt.block_on(async move {
                        // создаем инстанс сервера и в стейт передаем текущий контекст и ссылку на чаилда
                        let server = WarpServer::new(WarpServerState{
                            ctx: Arc::new(ctx),
                            associated_child: server_child,
                        });
                        server.listen(([127, 0, 0, 1], 8081)).await;
                    });
                };
                // запускаем таску с сервером и блокируемся пока она не завершиться
                run!(warp_task);
                Ok(())
            }
        })
    });

    Bastion::start();
    Bastion::block_until_stopped();
}

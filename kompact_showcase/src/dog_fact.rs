

pub mod dog_fact {
    use std::sync::Arc;
    use kompact::prelude::*;
    use tokio::runtime;
    use std::collections::HashMap;
    use std::time::Instant;
    use serde::Deserialize;


    // Компонент отвечает за получение рандомного факта о собаках
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

        // При получении локального сообщения отправляем запрос на получение рандомного факта и отвечаем с текстом полученного факта
        fn receive_local(&mut self, msg: Self::Message) -> Handled {
            self.spawn_local(move |async_self| async move {
                let now = Instant::now();
                // создаем рантайм который привязан к текущему потоку выполнения
                let rt = runtime::Builder::new_current_thread().enable_all().build().unwrap();
                // получаем ссылку на асинхронную версию указателя на текущий компонен
                let safe_async_self = Arc::new(async_self);
                let cloned_async_self = safe_async_self.clone();
                // запускаем в рантайме http-сlient и выполняем запрос после чего блокируемся и ожидаем ответ
                let res = rt.block_on(async move {
                    debug!(&cloned_async_self.log(), "From Tokio runtime");
                    DogFact::random().await
                }).unwrap();
                let elapsed = now.elapsed();
                debug!(safe_async_self.log(), "Time to perform logic: {}", elapsed.as_millis());
                dbg!(&res);
                // отправляем полученный факт в качестве ответа
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



    pub struct DogFact;

    impl DogFact {
        // запрос на получение рандомного факта о собаках
        pub async fn random() -> Result<String, Box<dyn std::error::Error>> {
            let now = Instant::now();
            let resp = reqwest::get("http://dog-api.kinduff.com/api/facts?number=1")
                .await?
                .json::<RandomDogFact>()
                .await?;
            let fact = String::from(&resp.facts[0]);
            let elapsed = now.elapsed();
            dbg!(format!("Time to perform request: {}", elapsed.as_millis()));
            dbg!(&fact);
            Ok(fact)
        }
    }



    #[derive(Debug, Deserialize)]
    struct RandomDogFact {
        facts: Vec<String>,
        success: bool,
    }
}




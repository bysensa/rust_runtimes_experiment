

use kompact::prelude::*;
use kompact_showcase::dog_fact::dog_fact::{DogFactComponent, DogFactRequest};
use kompact_showcase::tide_server::tide_server::TideServerComponent;
use kompact_showcase::warp_server::warp_server::WarpServerComponent;

fn main() {

    let mut config = KompactConfig::default();
    let system = config.build().expect("system");

    let dog_fact = system.create(DogFactComponent::new);
    let tide_server = system.create(TideServerComponent::new);
    let warp_server = system.create(WarpServerComponent::new);


    system.start(&tide_server);
    system.start(&warp_server);
    system.start(&dog_fact);


    let dog_fact_ref = dog_fact.actor_ref().hold().expect("live");
    let answer = dog_fact_ref.ask(Ask::of(DogFactRequest));
    let handle = system.spawn(async move {
        let result = answer.await.unwrap();
        dbg!(result.0);
    });

    system.await_termination();
}

use std::io;
use std::net::{SocketAddr};
use tide::{Request, Server};
use tide::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[derive(Debug)]
pub struct TideServer {
     server: Server<()>,
}

impl TideServer {
    pub fn new() -> TideServer {
        let mut app = tide::new();
        app.at("/").get(order_shoes);
        TideServer { server: app }
    }

    pub async fn listen(self, addr: &str ) -> io::Result<()> {
        self.server.listen(addr).await
    }
}


async fn order_shoes(mut req: Request<()>) -> tide::Result {
    // let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", "somebody", 2).into())
}
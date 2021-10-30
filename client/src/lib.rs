use std::collections::HashMap;
use std::time::Instant;
use serde::Deserialize;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub struct DogFact;

impl DogFact {
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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
  message: String,
}

fn main() -> reqwest::Result<()> {
  let url = "http://localhost:9000";
  let message: Message = reqwest::blocking::get(url)?.json()?;
  println!("GET {} - Response: {:?}", url, message);
  let client = reqwest::blocking::Client::new();
  let response = client.post(url).json(&message).send()?.text()?;
  println!("POST {:?} to {} - Response: {:?}", message, url, response);
  Ok(())
}

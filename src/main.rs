use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
  message: String,
}

fn main() -> reqwest::Result<()> {
  let url = "http://localhost:9000";

  println!("GET {}", url);

  let message: Message = reqwest::blocking::get(url)?.json()?;

  println!("{:?}", message);
  Ok(())
}

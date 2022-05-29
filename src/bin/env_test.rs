extern crate dotenv;

use dotenv::dotenv;
use std::env;


fn main() {
  dotenv().ok();

  println!("{}", env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not present"));
}
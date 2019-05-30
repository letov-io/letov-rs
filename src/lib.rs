extern crate serde_json;
extern crate rand;

use serde_json::Value;
use rand::Rng;

static TEXT_FILE: &'static str = include_str!("./output.json");

pub fn say () -> std::io::Result<String>  {
  let lyrics: Value = serde_json::from_str(&TEXT_FILE)?;
  let song = rand::thread_rng().gen_range(0, lyrics.as_array().unwrap().len());
  let string = rand::thread_rng().gen_range(0, lyrics[song]["lyrics"].as_array().unwrap().len());
  Ok(lyrics[song]["lyrics"][string].to_string())
}

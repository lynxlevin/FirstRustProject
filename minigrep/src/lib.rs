use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query = args[1].clone(); // there's a better way than cloning, in Chapter13
    let filename = args[2].clone(); // there's a better way than cloning, in Chapter13

    Ok(Config { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  println!("With text:\n{}", contents);

  Ok(())
}

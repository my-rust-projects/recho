use recho::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap();

    recho::run(config).unwrap();
}

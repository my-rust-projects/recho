pub struct Config {
    pub contents: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let contents = Vec::from(args);

        Ok(Config { contents })
    }
}

pub fn run(config: Config) -> Result<(), ()> {
    let mut contents = config.contents;
    let mut full_str = String::from("");

    contents.remove(0);

    if contents.len() == 0 {
        println!("");
    } else {
        for line in contents {
            full_str = full_str + " " + &line;
        }
    }
    println!("{}", full_str);
    Ok(())
}

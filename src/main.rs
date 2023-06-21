use std::env;
use std::error::Error;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    // for arguments in env::args() {
    //     println! ("{arguments}");
    // }

    let config: Config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    let query = &config.query;
    let filename = &config.filename;

    println!("{:?}", args);
    println!("Searching for {}", { query });
    println!("In file  {}", { filename });

    if let Err(e) = run(config) {
        println!("Application error {}", e);
        process::exit(1)
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("Searching for {}", contents);
    Ok(())
}
struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Please enter two arguments. See docs for instructions");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

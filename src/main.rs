use std::env;
use std::process;

use mini_grepp::Config;

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

    if let Err(e) = mini_grepp::run(config) {
        println!("Application error {}", e);
        process::exit(1)
    }
}

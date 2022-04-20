use branchify;
use std::env;
use std::io;

use branchify::{run, Config};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = Config::new(&args);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            print!("{}", run(config, &input))
        }
        Err(_) => print!("{}", ""),
    }
}

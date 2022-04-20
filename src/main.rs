use branchify;
use std::env;
use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            print!("{}", process_input(&input))
        }
        Err(_) => print!("{}", ""),
    }
}

fn process_input(input: &str) -> String {
    let branch_type = parse_options().branch_type;
    match input.trim().to_string().split_once("\t") {
        Some((a, b)) => branchify::get_branch_name(branch_type, a, b),
        None => panic!("No delimiter found"),
    }
}

struct Options {
    branch_type: String,
}

impl Options {
    fn new(branch_type: String) -> Self {
        Self {
            branch_type: branch_type,
        }
    }
}

fn parse_options() -> Options {
    let mut args = env::args();
    let branch_type: String;

    // ignoring the first argument
    args.next();
    match args.next() {
        Some(arg) => branch_type = arg,
        None => branch_type = "feature".to_string(),
    }
    Options::new(branch_type)
}

#[test]
fn test_process_input() {
    let expected = "adam/feature/APM-123-do-something-that-h";
    assert_eq!(process_input("APM-123\tDo something that helps"), expected);
}

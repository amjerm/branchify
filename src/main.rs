use std::io;
use branchify;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => { print!("{}", process_input(&input)) },
        Err(_) => print!("{}", ""),
    }
}

fn process_input(input: &str) -> String {
    match input.trim().to_string().split_once("\t") {
        Some((a, b)) => branchify::get_branch_name(a, b),
        None => panic!("No delimiter found"),
    }
}

#[test]
fn test_process_input() {
    let expected = "adam/feature/APM-123-do-something-that-h";
    assert_eq!(process_input("APM-123\tDo something that helps"), expected);
}

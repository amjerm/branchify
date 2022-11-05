//! # branchify
//!
//! A CLI tool for creating git branch names.

use regex::Regex;

pub struct Config {
    branch_type: String,
    prepended_value: String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let mut branch_type = String::from("feature");
        let mut prepended_value = String::from("");

        let arguments = args.clone();
        for (i, argument) in args.iter().enumerate() {
            match argument.as_str() {
                "-t" | "--type" => match arguments.get(i + 1) {
                    Some(value) => branch_type = value.clone(),
                    None => {
                        println!("No value provided for argument {}", argument);
                        std::process::exit(1);
                    }
                },
                "-p" | "--prepend" => match arguments.get(i + 1) {
                    Some(value) => prepended_value = value.clone(),
                    None => {
                        println!("No value provided for argument {}", argument);
                        std::process::exit(1);
                    }
                },
                _ => (),
            }
        }

        Self {
            branch_type,
            prepended_value,
        }
    }
}

pub fn run(config: Config, input: &str) -> Result<String, &'static str> {
    match input.trim().to_string().split_once("\t") {
        Some((a, b)) => Ok(get_branch_name(config, a, b)),
        None => return Err("Invalid input"),
    }
}

fn get_branch_name(config: Config, a: &str, b: &str) -> String {
    let branch_name = format!(
        "{}/{}-{}",
        get_prefix(config),
        a,
        b.to_lowercase().replace(" ", "-").replace("/", "-")
    );
    clean_branch_name(branch_name).to_string()
}

fn get_prefix(config: Config) -> String {
    let mut prefix = config.prepended_value;
    if prefix.len() > 0 {
        prefix.push('/');
    }
    prefix.push_str(config.branch_type.as_str());
    prefix
}

fn clean_branch_name(branch_name: String) -> String {
    let mut result = Regex::new(r"^/|\.\.|`|@\{|\.lock|\\|[\$\*\?'\[\]\(\)\^\~=<>@]")
        .unwrap()
        .replace_all(&branch_name, "")
        .to_string();
    result = Regex::new(r"//|/\.")
        .unwrap()
        .replace_all(&result, "/")
        .to_string();
    let mut re = Regex::new(r"-{2,}").unwrap();
    result = re
        .replace_all(&truncate_branch_name(result), "-")
        .to_string();
    re = Regex::new(r"[-\./]$").unwrap();
    re.replace_all(&result, "").to_string()
}

fn truncate_branch_name(branch_name: String) -> String {
    match branch_name.get(..40) {
        Some(s) => s.to_string(),
        None => branch_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let args: Vec<String> = vec![
            String::from("-p"),
            String::from("fx"),
            String::from("-t"),
            String::from("zammo"),
        ];
        let config = Config::new(&args);
        let expected = "fx/zammo/APM-123-do-something-that-helps";
        assert_eq!(
            run(config, "APM-123\tDo something that helps the product").unwrap(),
            expected
        );
    }

    #[test]
    fn test_run_with_long_branch_name() {
        let args: Vec<String> = vec![
            String::from("-p"),
            String::from("fx"),
            String::from("-t"),
            String::from("zammo"),
        ];
        let config = Config::new(&args);
        let expected = "fx/zammo/APM-123-do-something-that-helps";
        assert_eq!(
            run(
                config,
                "APM-123\tDo something that helps the product a whole whole lot"
            )
            .unwrap(),
            expected
        );
    }

    #[test]
    fn test_run_without_args() {
        let args: Vec<String> = vec![];
        let config = Config::new(&args);
        let expected = "feature/APM-123-do-something-that-helps";
        assert_eq!(
            run(config, "APM-123\tDo something that helps the product").unwrap(),
            expected
        );
    }

    #[test]
    fn test_run_with_type_arg_only() {
        let args: Vec<String> = vec![String::from("-t"), String::from("hotfix")];
        let config = Config::new(&args);
        let expected = "hotfix/APM-123-do-something-that-helps-t";
        assert_eq!(
            run(config, "APM-123\tDo something that helps the product").unwrap(),
            expected
        );
    }

    #[test]
    fn test_run_with_prepend_arg_only() {
        let args: Vec<String> = vec![String::from("-p"), String::from("adam")];
        let config = Config::new(&args);
        let expected = "adam/feature/APM-123-do-something-that-h";
        assert_eq!(
            run(config, "APM-123\tDo something that helps the product").unwrap(),
            expected
        );
    }

    #[test]
    fn test_run_with_non_word_chars() {
        let args: Vec<String> = vec![];
        let config = Config::new(&args);
        let expected = "feature/APM-123-do-somethin-that-_";
        assert_eq!(
            run(
                config,
                "APM-123\tDo ==> som'ethin' -> that -- _\"helps\"_ -- the product"
            )
            .unwrap(),
            expected
        );
    }

    #[test]
    fn test_run_with_other_unsupported_git_patterns() {
        // cannot end with .
        // cannot contain ..
        // cannot contain .lock
        // removes @
        let args: Vec<String> = vec![];
        let config = Config::new(&args);
        let expected = "feature/APM-123-do-something-that-helps";
        assert_eq!(
            run(config, "APM-123\tDo somethi@ng th..at he.locklps.").unwrap(),
            expected
        );
    }

    #[test]
    fn test_run_with_yet_other_unsupported_git_patterns() {
        // cannot end with /
        // cannot contain //
        // allows .
        let args: Vec<String> = vec![];
        let config = Config::new(&args);
        let expected = "feature/APM-123-do-some-thing-v9.1.1";
        assert_eq!(
            run(config, "APM-123\tDo some//thing v9.1.1/").unwrap(),
            expected
        );
    }

    #[test]
    fn test_run_with_back_ticks() {
        // cannot end with /
        // cannot contain //
        // allows .
        let args: Vec<String> = vec![];
        let config = Config::new(&args);
        let expected = "feature/APM-123-do-something-v9.1.1";
        assert_eq!(
            run(config, "APM-123\tDo `something` v9.1.1/").unwrap(),
            expected
        );
    }
}

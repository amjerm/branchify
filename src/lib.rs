pub struct Config {
    branch_type: String,
    prepended_value: String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let mut branch_type = String::from("feature");
        let mut prepended_value = String::from("");

        for (i, argument) in args.iter().enumerate() {
            match argument.as_str() {
                "-t" | "--type" => {
                    branch_type = args[i + 1].clone();
                }
                "-p" | "--prepend" => {
                    prepended_value = args[i + 1].clone();
                }
                _ => (),
            }
        }

        Self {
            branch_type,
            prepended_value,
        }
    }
}

pub fn run(config: Config, input: &str) -> String {
    match input.trim().to_string().split_once("\t") {
        Some((a, b)) => get_branch_name(config, a, b),
        None => panic!("No delimiter found"),
    }
}

pub fn get_branch_name(config: Config, a: &str, b: &str) -> String {
    let branch_name = format!(
        "{}/{}-{}",
        get_prefix(config),
        a,
        b.to_lowercase().replace(" ", "-")
    );
    truncate_branch_name(&branch_name).to_string()
}

fn get_prefix(config: Config) -> String {
    let mut prefix = config.prepended_value;
    if prefix.len() > 0 {
        prefix.push('/');
    }
    prefix.push_str(config.branch_type.as_str());
    prefix
}

fn truncate_branch_name(branch_name: &str) -> &str {
    match branch_name.get(..40) {
        Some(s) => s,
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
            run(config, "APM-123\tDo something that helps the product"),
            expected
        );
    }

    #[test]
    fn test_get_branch_name() {
        let args: Vec<String> = vec![
            String::from("-p"),
            String::from("fx"),
            String::from("-t"),
            String::from("zammo"),
        ];
        let config = Config::new(&args);
        let expected = "fx/zammo/APM-123-do-something-that-helps";
        assert_eq!(
            get_branch_name(config, "APM-123", "Do something that helps the product"),
            expected
        );
    }

    #[test]
    fn test_get_prefix() {
        let args: Vec<String> = vec![
            String::from("-p"),
            String::from("fx"),
            String::from("-t"),
            String::from("zammo"),
        ];
        let config = Config::new(&args);
        assert_eq!(get_prefix(config), "fx/zammo");
    }

    #[test]
    fn test_truncate_branch_name() {
        let branch_name = "adam/feature/APM-123-do-something-that-helps";
        assert_eq!(
            truncate_branch_name(branch_name),
            "adam/feature/APM-123-do-something-that-h"
        );
    }

    #[test]
    fn test_truncate_short_branch_name() {
        let branch_name = "adam/feature/APM-123-do";
        assert_eq!(truncate_branch_name(branch_name), "adam/feature/APM-123-do");
    }
}

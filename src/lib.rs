pub fn get_branch_name(a: &str, b: &str) -> String {
    let branch_name = format!("{}/{}-{}", get_prefix(), a, b.to_lowercase().replace(" ", "-"));
    truncate_branch_name(&branch_name).to_string()
}

fn get_prefix() -> String {
    String::from("adam/feature")
}

fn truncate_branch_name(branch_name: &str) -> &str  {
    match branch_name.get(..40) {
        Some(s) => s,
        None => branch_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_branch_name() {
        let expected = "adam/feature/APM-123-do-something-that-h";
        assert_eq!(get_branch_name("APM-123", "Do something that helps"), expected);
    }

    #[test]
    fn test_get_prefix() {
        assert_eq!(get_prefix(), "adam/feature");
    }

    #[test]
    fn test_truncate_branch_name() {
        let branch_name = "adam/feature/APM-123-do-something-that-helps";
        assert_eq!(truncate_branch_name(branch_name), "adam/feature/APM-123-do-something-that-h");
    }

    #[test]
    fn test_truncate_short_branch_name() {
        let branch_name = "adam/feature/APM-123-do";
        assert_eq!(truncate_branch_name(branch_name), "adam/feature/APM-123-do");
    }
}

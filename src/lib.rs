pub fn get_branch_name(branch_type: String, a: &str, b: &str) -> String {
    let branch_name = format!(
        "{}/{}-{}",
        get_prefix(branch_type),
        a,
        b.to_lowercase().replace(" ", "-")
    );
    truncate_branch_name(&branch_name).to_string()
}

fn get_prefix(branch_type: String) -> String {
    let mut name_segment = "adam/".to_owned();
    name_segment.push_str(branch_type.as_str());
    name_segment
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
    fn test_get_branch_name() {
        let expected = "adam/feature/APM-123-do-something-that-h";
        assert_eq!(
            get_branch_name(
                String::from("feature"),
                "APM-123",
                "Do something that helps"
            ),
            expected
        );
    }

    #[test]
    fn test_get_prefix() {
        assert_eq!(get_prefix(String::from("feature")), "adam/feature");
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

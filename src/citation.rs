use regex::Regex;

/// Extracts FUR citations of the form:
/// [FUR:<message_id>]
pub fn extract_fur_citations(response: &str) -> Vec<String> {
    let re = Regex::new(r"\[FUR:([a-f0-9\-]+)\]").unwrap();

    re.captures_iter(response)
        .map(|cap| cap[1].to_string())
        .collect()
}

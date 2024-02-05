use super::Asset;
use once_cell::sync::Lazy;
use regex::Regex;

/// Uses regex to find [Obsidian callouts](https://help.obsidian.md/Editing+and+formatting/Callouts)
/// and replaces them with appropriate HTML rendering
pub fn render(content: &str) -> Result<String, mdbook::errors::Error> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?m)^> \[!(?P<kind>[^\]]+)\]\s*$(?P<body>(?:\n>.*)*)")
            .expect("failed to parse regex")
    });
    let callouts = Asset::get("templates/callouts.html").expect("template not found");
    let callouts = std::str::from_utf8(callouts.data.as_ref())?;
    let content = RE.replace_all(content, |caps: &regex::Captures| {
        let kind = caps
            .name("kind")
            .expect("kind not found in regex")
            .as_str()
            .to_lowercase();
        let body = caps
            .name("body")
            .expect("body not found in regex")
            .as_str()
            .replace("\n>\n", "\n\n")
            .replace("\n> ", "\n");
        callouts.replace("{kind}", &kind).replace("{body}", &body)
    });
    Ok(content.into())
}

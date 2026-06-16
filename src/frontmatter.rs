use std::collections::BTreeSet;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Frontmatter {
    pub tags: BTreeSet<String>,
}

pub fn parse_file(path: &Path) -> Option<Frontmatter> {
    let content = std::fs::read_to_string(path).ok()?;
    parse_content(&content)
}

fn parse_content(content: &str) -> Option<Frontmatter> {
    let content = content.trim_start_matches('\u{feff}');

    if !content.starts_with("---") {
        return None;
    }

    let end = content[4..].find("---")? + 4;
    let yaml = &content[4..end];

    parse_tags(yaml)
}

fn parse_tags(yaml: &str) -> Option<Frontmatter> {
    let mut tags = BTreeSet::new();
    let mut in_tags_block = false;

    for line in yaml.lines() {
        let trimmed = line.trim();

        if let Some(value) = trimmed.strip_prefix("tags:") {
            let value = value.trim();

            if value.starts_with('[') && value.ends_with(']') {
                // Inline array: tags: [tag1, tag2]
                let inner = &value[1..value.len() - 1];
                for tag in inner.split(',') {
                    let tag = tag.trim().trim_matches('"').trim_matches('\'');
                    if !tag.is_empty() {
                        tags.insert(tag.to_string());
                    }
                }
                in_tags_block = false;
            } else if !value.is_empty() {
                // Single value: tags: tag1
                tags.insert(value.to_string());
                in_tags_block = false;
            } else {
                // Multi-line list follows
                in_tags_block = true;
            }
        } else if in_tags_block && trimmed.starts_with("- ") {
            let tag = trimmed[2..].trim().trim_matches('"').trim_matches('\'');
            if !tag.is_empty() {
                tags.insert(tag.to_string());
            }
        } else if !trimmed.is_empty() && !trimmed.starts_with('#') {
            // Non-empty, non-comment line ends tags block
            in_tags_block = false;
        }
    }

    if tags.is_empty() {
        None
    } else {
        Some(Frontmatter { tags })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_array() {
        let content = "---\ntags: [rust, wasm]\n---\n# Title\n";
        let fm = parse_content(content).unwrap();
        assert!(fm.tags.contains("rust"));
        assert!(fm.tags.contains("wasm"));
    }

    #[test]
    fn test_multiline_list() {
        let content = "---\ntags:\n  - rust\n  - wasm\n---\n# Title\n";
        let fm = parse_content(content).unwrap();
        assert!(fm.tags.contains("rust"));
        assert!(fm.tags.contains("wasm"));
    }

    #[test]
    fn test_no_frontmatter() {
        let content = "# Title\n\nNo frontmatter here.\n";
        assert!(parse_content(content).is_none());
    }

    #[test]
    fn test_no_tags() {
        let content = "---\ntitle: Hello\n---\n# Title\n";
        assert!(parse_content(content).is_none());
    }
}

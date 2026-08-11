//! Collector for reference-style link definitions.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Shared handle for passing the collector through the conversion context.
pub type ReferenceCollectorHandle = Rc<RefCell<ReferenceCollector>>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct ReferenceKey {
    url: String,
    title: Option<String>,
}

/// Collects link/image references during conversion and produces a reference
/// definitions section at the end of the document.
#[derive(Debug, Default)]
pub struct ReferenceCollector {
    map: HashMap<ReferenceKey, usize>,
    entries: Vec<(usize, String, Option<String>)>,
}

impl ReferenceCollector {
    /// Create a new, empty reference collector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a URL (and optional title) and return its 1-based reference number.
    ///
    /// If the same URL+title pair was already registered, the existing number is returned.
    pub fn get_or_insert(&mut self, url: &str, title: Option<&str>) -> usize {
        let key = ReferenceKey {
            url: url.to_string(),
            title: title.map(String::from),
        };
        if let Some(&num) = self.map.get(&key) {
            return num;
        }
        let num = self.entries.len() + 1;
        self.map.insert(key, num);
        self.entries.push((num, url.to_string(), title.map(String::from)));
        num
    }

    /// Produce the reference definitions section.
    ///
    /// Returns an empty string when no references were collected.
    pub fn finish(&self) -> String {
        if self.entries.is_empty() {
            return String::new();
        }
        let mut out = String::new();
        for (num, url, title) in &self.entries {
            out.push('[');
            out.push_str(&num.to_string());
            out.push_str("]: ");
            out.push_str(url);
            if let Some(t) = title {
                out.push_str(" \"");
                out.push_str(&crate::converter::inline::link::escape_markdown_title(t));
                out.push('"');
            }
            out.push('\n');
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::ReferenceCollector;

    #[test]
    fn finish_escapes_a_trailing_backslash_in_title_so_the_closing_quote_is_not_swallowed() {
        // ~keep audit #24 finding 8: a title ending in a literal `\` must not let a following `\"`
        // read as an escaped quote instead of the closing delimiter, which would otherwise
        // leave the reference definition (and everything after it) unterminated.
        let mut collector = ReferenceCollector::new();
        collector.get_or_insert("https://example.com", Some("foo\\"));
        assert_eq!(collector.finish(), "[1]: https://example.com \"foo\\\\\"\n");
    }
}

// ~keep reason: CLI application modules do not expose docs to users; doc coverage not required
#![allow(missing_docs)]

use encoding_rs::Encoding;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use std::time::Duration;

pub const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (compatible; html-to-markdown-cli/2.10; +https://github.com/xberg-io/html-to-markdown)";

pub fn decode_bytes(bytes: &[u8], encoding_name: &str) -> Result<String, String> {
    let lowercase = encoding_name.to_lowercase();
    let normalized = match lowercase.as_str() {
        "latin-1" | "latin1" => "iso-8859-1",
        "latin-2" | "latin2" => "iso-8859-2",
        "latin-3" | "latin3" => "iso-8859-3",
        "latin-4" | "latin4" => "iso-8859-4",
        "latin-5" | "latin5" => "iso-8859-5",
        "latin-6" | "latin6" => "iso-8859-6",
        "latin-7" | "latin7" => "iso-8859-7",
        "latin-8" | "latin8" => "iso-8859-8",
        "latin-9" | "latin9" => "iso-8859-9",
        "latin-10" | "latin10" => "iso-8859-10",
        _ => encoding_name,
    };

    let encoding =
        Encoding::for_label(normalized.as_bytes()).ok_or_else(|| format!("Unknown encoding '{encoding_name}'"))?;

    let (decoded, _, had_errors) = encoding.decode(bytes);
    if had_errors {
        tracing::warn!(encoding = %encoding_name, "some characters could not be decoded correctly");
    }
    Ok(decoded.into_owned())
}

pub fn extract_charset(content_type: &str) -> Option<String> {
    content_type
        .split(';')
        .map(str::trim)
        .find_map(|part| part.strip_prefix("charset=").map(|v| v.trim_matches('"').to_string()))
}

pub fn fetch_url(url: &str, user_agent: &str, default_encoding: &str) -> Result<String, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {e}"))?;

    let response = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send()
        .map_err(|e| format!("Failed to fetch '{url}': {e}"))?;

    let status = response.status();
    if !status.is_success() {
        return Err(format!("Request failed for '{url}': HTTP {status}"));
    }

    let charset = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(extract_charset);

    let bytes = response
        .bytes()
        .map_err(|e| format!("Failed to read response body from '{url}': {e}"))?;

    let encoding_name = charset.as_deref().unwrap_or(default_encoding);
    decode_bytes(&bytes, encoding_name)
}

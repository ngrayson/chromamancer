//! Strip `//` and `/* */` comments so serde_json can parse theme / mapping files.

use std::io::Read;

pub fn parse_jsonc(s: &str) -> serde_json::Result<serde_json::Value> {
    let mut stripped = Vec::new();
    json_comments::StripComments::new(s.as_bytes())
        .read_to_end(&mut stripped)
        .map_err(|e| serde_json::Error::io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    serde_json::from_slice(&stripped)
}

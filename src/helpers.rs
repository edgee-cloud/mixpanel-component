use regex::Regex;
use std::collections::HashMap;

pub fn insert_if_nonempty(map: &mut HashMap<String, String>, key: &str, value: &str) {
    if !value.trim().is_empty() {
        map.insert(key.to_string(), value.to_string());
    }
}

pub fn parse_browser_info(user_agent: &str) -> (Option<String>, Option<String>) {
    let browser_regexes = vec![
        (r"(Firefox)/([\d\.]+)", "Firefox"),
        (r"(Edg)/([\d\.]+)", "Edge"),
        (r"(Chrome)/([\d\.]+)", "Chrome"),
        (r"(Safari)/([\d\.]+)", "Safari"),
        (r"(Opera)/([\d\.]+)", "Opera"),
    ];

    for (pattern, name) in browser_regexes {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(user_agent) {
                let version = caps.get(2).map(|m| m.as_str().to_string());
                return (Some(name.to_string()), version);
            }
        }
    }

    (None, None)
}

pub fn mixpanel_endpoint(region: &str) -> String {
    format!("https://{}.mixpanel.com", region)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_if_nonempty_works() {
        let mut map = HashMap::new();
        insert_if_nonempty(&mut map, "key", "value");
        assert_eq!(map.get("key"), Some(&"value".to_string()));

        insert_if_nonempty(&mut map, "empty", "");
        assert!(!map.contains_key("empty"));
    }

    #[test]
    fn parse_browser_info_detects_chrome() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
        let (name, version) = parse_browser_info(ua);
        assert_eq!(name, Some("Chrome".to_string()));
        assert_eq!(version, Some("120.0.0.0".to_string()));
    }

    #[test]
    fn mixpanel_endpoint_works() {
        let region = "api-eu";
        let endpoint = mixpanel_endpoint(region);
        assert_eq!(endpoint, "https://api-eu.mixpanel.com");
    }
}

use emojis::Emoji;
use html::metadata::builders::HeadBuilder;
use html::metadata::Head;
use html::root::builders::BodyBuilder;
use html::root::{Body, Html};
use serde::Deserialize;
use serde_json;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;

/// Gracefully converts an emoji shortcode to the string representation of the unicode character.
///
/// # Examples
///
/// ```rust
/// assert_eq!("1f600", webicons::normalize_id("grinning", "emojis"));
/// ```
pub fn normalize_id(id: &str, family: &str) -> String {
    if family == "emojis" && !unic_emoji_char::is_emoji(str_to_char(&id)) {
        get_id_from_shortcode(id)
    } else {
        String::from(id)
    }
}

/// Opens the config file and returns config object.
fn get_config(file_path: &str) -> MetadataConfig {
    let config_file = match File::open(Path::new(file_path)) {
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
        Ok(file) => file,
    };
    serde_json::from_reader(config_file).expect("Unable to deserialize JSON.")
}

/// Get the defalt value for vendor.
pub fn get_default_vendor(file_path: &str, family: &str) -> String {
    let config = get_config(file_path);
    if config.contains_key(family) {
        match config[family].keys().next_back() {
            Some(key) => key.to_string(),
            None => panic!["{} has no keys under [\"{}\"]!", file_path, family],
        }
    } else {
        panic!["{} does not contain [\"{}\"]!", file_path, family];
    }
}

/// Reads WebiconVendorMetadata of `family.vendor` from `file_path`.
pub fn get_metadata(file_path: &str, family: &str, vendor: &str) -> WebiconVendorMetadata {
    let config = get_config(file_path);
    if config.contains_key(family) {
        if config[family].contains_key(vendor) {
            config[family][vendor].clone()
        } else {
            panic![
                "{} does not have [\"{}\"] under [\"{}\"]!",
                file_path, vendor, family
            ];
        }
    } else {
        panic!["{} does not contain [\"{}\"]!", file_path, family];
    }
}

/// Metadata about a webicon vendor.
#[derive(Deserialize, Debug, Clone)]
pub struct WebiconVendorMetadata {
    pub name: String,
    pub attribution: String,
    pub license_name: String,
    pub license_url: String,
    pub url: String,
}

/// A WebiconVendor creates a certain set of emojis or icons.
/// For more info, see https://emojipedia.org/vendors/
pub type WebiconVendor = BTreeMap<String, WebiconVendorMetadata>;

/// MetadataConfig represents the full configuration object for all webicon vendors.
pub type MetadataConfig = BTreeMap<String, WebiconVendor>;

pub fn make_body(metadata: &WebiconVendorMetadata) -> Body {
    let mut body: BodyBuilder = Body::builder();

    let h1 = format!("<h1>{}</h1>", metadata.name);
    let attribution = format!("<p>{}</p>", metadata.attribution);
    let url = format!("<p><a href=\"{url}\">{url}</a></p>", url = metadata.url);
    let license = format!(
        "<p>License: <a href=\"{}\">{}</a></p>",
        metadata.license_url, metadata.license_name
    );

    body.text(h1);
    body.text(url);
    body.text(attribution);
    body.text(license);

    body.build()
}

pub fn make_head(title: &str) -> Head {
    let mut head: HeadBuilder = Head::builder();
    head.title(|t| t.text(String::from(title)));
    head.link(|l| {
        l.rel("icon")
            .type_("image/x-icon")
            .href("/favicon.ico")
            .sizes("any")
    });
    head.build()
}

pub fn make_html(metadata: &WebiconVendorMetadata, title: &str) -> Html {
    Html::builder()
        .push(make_head(title))
        .push(make_body(metadata))
        .build()
}

/// Gets a string of an emoji given its ID.
///
/// # Examples
///
/// ```rust
/// assert_eq!("😀", webicons::get_emoji_string_from_id("1f600"));
/// ```
pub fn get_emoji_string_from_id(id: &str) -> String {
    if unic_emoji_char::is_emoji(str_to_char(id)) {
        let i = u32::from_str_radix(id, 16).unwrap();
        String::from(char::from_u32(i).unwrap())
    } else {
        panic!("{} is not an emoji.", id);
    }
}

// Graceful getting of emoji from shortcode.
fn get_by_shortcode(shortcode: &str) -> &Emoji {
    let shortcodes: Vec<&str> = emojis::iter().map(|x| x.shortcodes()).flatten().collect();
    if shortcodes.contains(&shortcode) {
        emojis::get_by_shortcode(shortcode).unwrap()
    } else {
        panic!("Unable to find shortcode {}", shortcode);
    }
}

/// Returns "1f600" given "grinning".
pub fn get_id_from_shortcode(shortcode: &str) -> String {
    let emoji_str: &str = get_by_shortcode(shortcode).as_str();
    let emoji_unicode: char = str_to_char(emoji_str);
    format!("{:x}", emoji_unicode as u32)
}

/// Returns vec!["grinning"] given "😀".
pub fn get_shortcodes(e: &Emoji) -> Vec<&str> {
    e.shortcodes().collect::<Vec<&str>>()
}

/// Converts the first character of a str ("abc") to a char ('a').
pub fn str_to_char(s: &str) -> char {
    s.chars().nth(0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shortcodes() {
        let emoji_grinning = emojis::get("😀").unwrap();
        assert_eq!(vec!["grinning"], get_shortcodes(emoji_grinning));
    }

    #[test]
    fn test_str_to_char() {
        assert_eq!('a', str_to_char("a"));
        assert_eq!('😀', str_to_char("😀"));
        assert_eq!('\u{1f600}', str_to_char("\u{1f600}"));
    }
}

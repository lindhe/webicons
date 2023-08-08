use emojis::Emoji;
use html::metadata::builders::HeadBuilder;
use html::metadata::Head;
use html::root::builders::BodyBuilder;
use html::root::{Body, Html};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata about a favicon set.
#[derive(Serialize, Deserialize, Debug)]
pub struct FaviconSetMetadata {
    pub name: String,
    pub attribution: String,
    pub license_name: String,
    pub license_url: String,
    pub url: String,
}

/// A FaviconSet can be for example "OpenMoji", "Noto" or "MaterialDesignIcons".
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum FaviconSet {
    Set(HashMap<String, FaviconSetMetadata>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct MetadataConfig {
    emojis: FaviconSet,
    icons: FaviconSet,
}

pub fn make_body(
    name: &str,
    url: &str,
    attribution: &str,
    license_name: &str,
    license_url: &str,
) -> Body {
    let mut body: BodyBuilder = Body::builder();

    let h1 = format!("<h1>{name}</h1>");
    let attribution_p = format!("<p>{attribution}</p>");
    let url_p = format!("<p><a href=\"{url}\">{url}</a></p>");
    let license_p = format!("<p>License: <a href=\"{license_url}\">{license_name}</a></p>");

    body.text(h1);
    body.text(url_p);
    body.text(attribution_p);
    body.text(license_p);

    body.build()
}

pub fn make_head() -> Head {
    let mut head: HeadBuilder = Head::builder();
    head.title(|t| t.text("FOO"));
    head.link(|l| {
        l.rel("icon")
            .type_("image/x-icon")
            .href("/favicon.ico")
            .sizes("any")
    });
    head.build()
}

pub fn make_html(
    name: &str,
    url: &str,
    attribution: &str,
    license_name: &str,
    license_url: &str,
) -> Html {
    Html::builder()
        .push(make_head())
        .push(make_body(name, url, attribution, license_name, license_url))
        .build()
}

/// Returns "😀" given "1f600".
pub fn get_emoji_string_from_id(id: &str) -> String {
    let i = u32::from_str_radix(id, 16).unwrap();
    String::from(char::from_u32(i).unwrap())
}

/// Returns "1f600" given "grinning".
pub fn get_id_from_shortcode(shortcode: &str) -> String {
    let emoji_str: &str = emojis::get_by_shortcode(shortcode).unwrap().as_str();
    let emoji_unicode: char = emoji_str.chars().nth(0).unwrap();
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
    fn test_get_emoji_string_from_id() {
        assert_eq!("😀", get_emoji_string_from_id("1f600"));
    }

    #[test]
    fn test_get_id_from_shortcode() {
        assert_eq!("1f600", get_id_from_shortcode("grinning"));
    }

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

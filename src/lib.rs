use emojis::Emoji;
use html::metadata::builders::HeadBuilder;
use html::root::builders::BodyBuilder;
use html::root::Html;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IconsetMetadata {
    pub name: String,
    pub attribution: String,
    pub license_name: String,
    pub license_url: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Emojis {
    pub open_moji: IconsetMetadata,
    pub noto: IconsetMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Icons {
    pub material_design_icons: IconsetMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub emojis: Emojis,
    pub icons: Icons,
}

pub fn make_body<'a>(b: &'a mut BodyBuilder) -> &mut BodyBuilder {
    let name = "<h1>OpenMoji</h1>";
    let attribution =
        "<p>All emojis designed by <a href=\"https://openmoji.org\">OpenMoji</a> – the open-source emoji and icon project.</p>";
    let license =
        "<p>License: <a href=\"https://creativecommons.org/licenses/by-sa/4.0/#\">CC BY-SA 4.0</a></p>";

    b.text(name);
    b.text(attribution);
    b.text(license);
    b
}

pub fn make_head<'a>(h: &'a mut HeadBuilder) -> &mut HeadBuilder {
    h.title(|t| t.text("FOO"));
    h.link(|l| {
        l.rel("icon")
            .type_("image/x-icon")
            .href("/favicon.ico")
            .sizes("any")
    });
    h
}

pub fn make_html() -> Html {
    Html::builder().head(make_head).body(make_body).build()
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

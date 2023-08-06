use emojis::Emoji;

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

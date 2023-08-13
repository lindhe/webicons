// vim: foldmethod=marker :

pub mod metadata {
    //! Helpers for handling metadata for webicons.
    //{{{

    use serde::Deserialize;
    use std::collections::BTreeMap;
    use std::fmt;
    use std::fs::File;
    use std::path::Path;
    use std::str::FromStr;

    /// Metadata about a webicon vendor.
    #[derive(Deserialize, Debug, Clone)]
    pub struct WebiconVendorMetadata {
        pub(crate) name: String,
        pub(crate) attribution: String,
        pub(crate) license_name: String,
        pub(crate) license_url: String,
        pub(crate) url: String,
    }

    // WebiconVendor {{{

    /// A WebiconVendor creates a certain set of emojis or icons.
    ///
    /// For more info, see <https://emojipedia.org/vendors/>
    type WebiconVendor = BTreeMap<String, WebiconVendorMetadata>;

    //}}}

    // WebiconFamily {{{

    /// Represents an allowed family of webicons.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum WebiconFamily {
        Emojis,
        Icons,
    }

    impl fmt::Display for WebiconFamily {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Emojis => write!(f, "emojis"),
                Self::Icons => write!(f, "icons"),
            }
        }
    }

    /// The error for bad (de)serialization of WebiconFamily.
    #[derive(Debug, PartialEq, Eq)]
    pub struct ParseWebiconFamilyError;

    impl FromStr for WebiconFamily {
        type Err = ParseWebiconFamilyError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "emojis" => Ok(Self::Emojis),
                "icons" => Ok(Self::Icons),
                _ => Err(ParseWebiconFamilyError),
            }
        }
    }

    //}}}

    // MetadataConfig {{{

    /// MetadataConfig represents the full configuration object for all webicon vendors.
    type MetadataConfig = BTreeMap<String, WebiconVendor>;

    //}}}

    /// Opens the config file and returns config object.
    fn get_config(file_path: &str) -> MetadataConfig {
        let config_file = match File::open(Path::new(file_path)) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open {}: {}", file_path, why),
        };
        serde_json::from_reader(config_file).expect("Unable to deserialize JSON.")
    }

    /// Get the default value for vendor.
    ///
    /// ```rust
    /// use webicons::metadata::get_default_vendor;
    /// let default_vendor = get_default_vendor("./config/metadata.json", "emojis");
    /// assert_eq!("OpenMoji", default_vendor);
    /// ```
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
    pub fn get_metadata(
        file_path: &str,
        family: &WebiconFamily,
        vendor: &str,
    ) -> WebiconVendorMetadata {
        let family = &family.to_string();
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

    //}}}
}

pub mod token {
    //! Helpers for handling misc webicon representations (strings, IDs, etc).
    //{{{

    use crate::metadata::WebiconFamily;
    use emojis::Emoji;

    /// Gracefully converts an emoji shortcode to the string representation of the unicode character.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use webicons::{token, metadata};
    ///
    /// assert_eq!("1f600", token::normalize_id("grinning", &metadata::WebiconFamily::Emojis));
    /// ```
    pub fn normalize_id(id: &str, family: &WebiconFamily) -> String {
        if family == &WebiconFamily::Emojis && !unic_emoji_char::is_emoji(str_to_char(&id)) {
            get_id_from_shortcode(id)
        } else {
            String::from(id)
        }
    }

    /// Gets an emoji given its ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!("😀", webicons::token::get_emoji_from_id("1f600").as_str());
    /// ```
    pub fn get_emoji_from_id(id: &str) -> &Emoji {
        let i = u32::from_str_radix(id, 16).unwrap();
        let emoji_string = String::from(char::from_u32(i).unwrap());
        emojis::get(&emoji_string).expect(&format!("Unable to get emoji from id {}.", id))
    }

    /// Gets an emoji's ID given its shortcode.
    fn get_id_from_shortcode(shortcode: &str) -> String {
        let emoji: &Emoji = match emojis::get_by_shortcode(shortcode) {
            Some(emoji) => emoji,
            None => panic!("Unable to find shortcode {}", shortcode),
        };
        format!("{:x}", str_to_char(emoji.as_str()) as u32)
    }

    /// Converts the first character of a str ("abc") to a char ('a').
    fn str_to_char(s: &str) -> char {
        s.chars().nth(0).unwrap()
    }

    #[cfg(test)]
    mod tests {
        //! Unit tests for private parts of token module.
        //{{{

        use super::*;

        #[test]
        fn test_str_to_char() {
            assert_eq!('a', str_to_char("a"));
            assert_eq!('a', str_to_char("abc"));
            assert_eq!('😀', str_to_char("😀"));
            assert_eq!('\u{1f600}', str_to_char("\u{1f600}"));
        }

        #[test]
        fn test_get_id_from_shortcode() {
            let id = get_id_from_shortcode("grinning");
            assert_eq!("1f600", id);
        }
        //}}}
    }
    //}}}
}

pub mod html {
    //! Helpers for creating HTML objects.
    //{{{

    use crate::metadata::WebiconVendorMetadata;
    use html::metadata::{builders::HeadBuilder, Head};
    use html::root::{builders::BodyBuilder, Body, Html};

    fn make_body(metadata: &WebiconVendorMetadata) -> Body {
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

    fn make_head(title: &str) -> Head {
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
    //}}}
}

#[cfg(test)]
mod tests {
    //! Unit tests for public traits.
    //{{{

    use super::metadata::WebiconFamily;
    use std::str::FromStr;

    #[test]
    fn test_webicon_family_traits() {
        assert_eq!("emojis", WebiconFamily::Emojis.to_string());
        assert_eq!(
            WebiconFamily::Emojis,
            WebiconFamily::from_str("emojis").unwrap()
        );
    }

    //}}}
}

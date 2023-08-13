use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::response::Redirect;
use std::path::Path;
use std::str::FromStr;
use webicons::*;

#[macro_use]
extern crate rocket;

const DEFAULT_CONFIG_FILE_PATH: &str = "./config/metadata.json"; // TODO: Get from env or argv

#[get("/emoji/<id>?<vendor>")]
fn emoji_redirect(id: &str, vendor: Option<String>) -> Redirect {
    // TODO: Update the call to get_webicon() to use WebiconFamily::Emojis instead of "emojis" as
    // soon as this bug is resolved: https://github.com/SergioBenitez/Rocket/issues/2595
    Redirect::to(uri!(get_webicon("emojis", id, vendor)))
}

#[get("/<family>/<id>?<vendor>")]
fn get_webicon(family: &str, id: &str, vendor: Option<String>) -> (ContentType, String) {
    let default_vendor = metadata::get_default_vendor(DEFAULT_CONFIG_FILE_PATH, family).to_string();
    let vendor = vendor.unwrap_or(default_vendor);
    let family = metadata::WebiconFamily::from_str(family).expect("Invalid webicon family.");
    let id = token::normalize_id(id, family);

    let metadata = metadata::get_metadata(DEFAULT_CONFIG_FILE_PATH, family, &vendor);
    let emoji = token::get_emoji_from_id(&id).as_str();
    let title = format!("{} ({})", emoji, id);
    let html = webicons::html::make_html(&metadata, &title);

    (ContentType::HTML, html.to_string())
}

#[get("/favicon.ico")]
async fn get_favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("favicons/favicon.ico"))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![emoji_redirect, get_webicon, get_favicon])
}

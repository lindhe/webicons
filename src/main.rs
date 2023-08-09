use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::response::Redirect;
use std::path::{Path, PathBuf};
use webicons::*;

#[macro_use]
extern crate rocket;

const DEFAULT_FAMILY: &str = "emojis";
const DEFAULT_CONFIG_FILE_PATH: &str = "./config/metadata.json"; // TODO: Get from env or argv

#[get("/emoji/<path..>")]
fn emoji_redirect(path: PathBuf) -> Redirect {
    let p = format!(
        "/emojis/{}",
        path.as_os_str()
            .to_str()
            .expect("TODO: Fix this mess.")
            .to_string()
    );
    Redirect::to(p)
}

#[get("/<family>/<id>?<vendor>")]
fn get_webicon(family: &str, id: &str, vendor: Option<String>) -> (ContentType, String) {
    let default_vendor = get_default_vendor(DEFAULT_CONFIG_FILE_PATH, DEFAULT_FAMILY).to_string();
    let vendor = vendor.unwrap_or(default_vendor);
    let id = normalize_id(id, family);

    // TODO: Remove these when things works with HTML.
    let emoji = get_emoji_string_from_id(&id);
    println!("TODO: Emoji with ID {}: {}", id, emoji);

    let metadata = get_metadata(DEFAULT_CONFIG_FILE_PATH, family, &vendor);
    let html = make_html(&metadata, &id);

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

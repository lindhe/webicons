use rocket::http::ContentType;
use webicons::*;

#[macro_use]
extern crate rocket;

#[get("/emoji/<id>")]
fn emoji(id: &str) -> String {
    let emoji = get_emoji_string_from_id(id);
    format!("TODO: Emoji with ID {}: {}", id, emoji)
}

#[get("/foo")]
fn foo() -> (ContentType, String) {
    let attribution = "All emojis designed by <a href=\"https://openmoji.org\">OpenMoji</a> – the open-source emoji and icon project.";
    let name = "OpenMoji";
    let url = "https://openmoji.org";
    let license_name = "CC BY-SA 4.0";
    let license_url = "https://creativecommons.org/licenses/by-sa/4.0/#";

    let html = make_html(name, url, attribution, license_name, license_url);

    (ContentType::HTML, html.to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![emoji, foo])
}

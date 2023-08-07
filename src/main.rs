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
    (ContentType::HTML, make_html().to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![emoji, foo])
}

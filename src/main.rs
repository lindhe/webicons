use webicons::*;

#[macro_use]
extern crate rocket;

#[get("/emoji/<id>")]
fn emoji(id: &str) -> String {
    format!("TODO: Return emoji with ID {}. 🙃", id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![emoji])
}

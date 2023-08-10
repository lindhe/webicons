use webicons::*;

fn main() {
    let metadata = get_metadata("./config/metadata.json", "emojis", "OpenMoji");
    let html = make_html(&metadata, "1f600");
    println!("{}", html.to_string());
}

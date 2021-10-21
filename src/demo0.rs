use std::fs;

pub(crate)
fn convert_md() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("fetching url:{}", url);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("Converting html to markdown...");

    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes()).unwrap();

    println!("Converted markdown has been saved in {}.", output);


}
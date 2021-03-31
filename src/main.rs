#![allow(dead_code)]

#[macro_use] extern crate derivative;
#[macro_use] extern crate serde;

use std::{fs::File, io::Write};

use cursive::{Rect, event::Key, traits::*, views::{FixedLayout, ScrollView}};
use cursive::views::{EditView, TextView};
use cursive::Cursive;

mod request;
use request::RequestTemplate;
mod response;
use response::Response;

fn main() -> anyhow::Result<()> {
	let request_url = get_request_url()?;
	println!("Connecting to: {}", request_url);
	let request_template = RequestTemplate::new("data/request_template.json", "data/multi_match_template.json", "data/wildcard_template.json")?;

	let request = request_template.template("i3");
	
	let mut request_output = File::create("test_output.json")?;
	request_output.write_all(&serde_json::to_vec(&request)?)?;
	/* let request_reader = BufReader::new(File::open("data/template_request.json")?);
	let request_json: Request = serde_json::from_reader(request_reader)?;
	println!("{:#?}", request_json); */

	/* render(); */
	Ok(())
}

fn get_request_url() -> anyhow::Result<String> {
	// Find Javascript file
	let site_text = reqwest::blocking::get("https://search.nixos.org/")?.text()?;
	let page_regex = regex::Regex::new(r#"src="(/main-.*\.js)""#)?;
	let capture = page_regex.captures_iter(&site_text).next().unwrap();

	// Find url from javascript file
	let javascript_text = reqwest::blocking::get(format!("https://search.nixos.org{}", &capture[1]))?.text()?;

	let script_regex = regex::Regex::new(r#"https://nixos-search.*\.bonsaisearch.net:443"#).unwrap();
	let capture = script_regex.captures_iter(&javascript_text).next().unwrap();
	Ok( capture[0].to_owned() )
}

fn render() {
	let mut siv = cursive::default();
	siv.add_global_callback(Key::Esc, |s| s.quit());

	let search_bar = EditView::new()
		.on_submit(search_input)
		.with_name("search-bar")
		.full_width();
	let search_results = TextView::new("Search Something...")
		.scrollable()
		.with_name("search-results");
	
	let width = 80;
	siv.add_layer(
		FixedLayout::new()
    	.child(Rect::from_size((0,0), (width,2)), search_bar)
    	.child(Rect::from_size((0,2), (width,14)), search_results)
	);

	siv.run();
}

// This will replace the current layer with a new popup.
// If the name is empty, we'll show an error message instead.
fn search_input(s: &mut Cursive, input: &str) {
	let mut results = s.find_name::<ScrollView<TextView>>("search-results").unwrap();
	let result_text = results.get_inner_mut();
	if !input.is_empty() {
		// Try again as many times as we need!
		let content = format!("You Searched: {}", input);
		result_text.set_content(content);
		//s.add_layer(Dialog::info(format!("Searching: {}", input)));
	};
}
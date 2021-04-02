#![allow(dead_code)]

#[macro_use] extern crate derivative;
#[macro_use] extern crate serde;
#[macro_use] extern crate lazy_static;

use cursive::{Rect, event::Key, traits::*, views::{FixedLayout, ScrollView}};
use cursive::views::{EditView, TextView};
use cursive::Cursive;
use regex::Regex;
use reqwest::header::HeaderMap;

mod request;
use request::RequestTemplate;
mod response;
use response::Response;

fn main() -> anyhow::Result<()> {
	//println!("Search Query: {:?}", query_search("hello there"));
	render();
	Ok(())
}

fn get_js_file() -> anyhow::Result<String> {
	// Find Javascript file
	let site_text = reqwest::blocking::get("https://search.nixos.org/")?.text()?;
	let page_regex = regex::Regex::new(r#"src="(/main-.*\.js)""#)?;
	let capture = page_regex.captures_iter(&site_text).next().unwrap();

	// Find url from javascript file
	let js_text = reqwest::blocking::get(format!("https://search.nixos.org{}", &capture[1]))?.text()?;
	Ok(js_text)
}
fn get_request_url(js_text: &str) -> anyhow::Result<String> {
	let script_regex = regex::Regex::new(r#"https://nixos-search.*\.bonsaisearch.net:443"#).unwrap();
	let capture = script_regex.captures_iter(&js_text).next().unwrap();
	Ok( capture[0].to_owned() )
}
fn get_auth<'a>(js_text: &'a str) -> anyhow::Result<(&'a str, &'a str)> {
	lazy_static! {
		static ref USERNAME_REGEX: Regex = Regex::new(r#"ELASTICSEARCH_USERNAME\|\|"(.*?)""#).unwrap();
	}
	let username = USERNAME_REGEX.captures_iter(&js_text).next().unwrap().get(1).unwrap().as_str();
	
	lazy_static! {
		static ref PASSWORD_REGEX: Regex = Regex::new(r#"ELASTICSEARCH_PASSWORD\|\|"(.*?)""#).unwrap();
	}
	let password = PASSWORD_REGEX.captures_iter(js_text).next().unwrap().get(1).unwrap().as_str();
	
	Ok((username, password))
}
fn query_search(search: &str) -> anyhow::Result<Response> {
	lazy_static! {
		static ref REQUEST_BUILDER: (String, HeaderMap) = {
			let js_file: String = get_js_file().unwrap();
			let request_url = get_request_url(&js_file).unwrap() + "/_search";
			let (user, pass) = get_auth(&js_file).unwrap();
			
			let client = reqwest::blocking::Client::new();
			let header_map = client.post(request_url.clone()).header(reqwest::header::CONTENT_TYPE, "application/json")
				.basic_auth(user, Some(pass)).build().unwrap().headers().clone();
			(request_url, header_map)
		};
		static ref REQUEST_TEMPLATE: RequestTemplate = RequestTemplate::new("data/request_template.json", "data/multi_match_template.json", "data/wildcard_template.json").unwrap();
	}
	let request = REQUEST_TEMPLATE.template(search);

	let request_string = serde_json::to_string(&request)?;
	let request_string = jsonxf::pretty_print(&request_string).unwrap();
	let request_string = std::fs::read_to_string("data/request_template.json")?;
	std::fs::write("target/request_check.json", &request_string)?;

	let client = reqwest::blocking::Client::new();
	let client_response = client.post(&REQUEST_BUILDER.0).headers(REQUEST_BUILDER.1.clone()).body(request_string).send()?;

	let response_string = String::from_utf8(client_response.bytes()?.to_vec())?;
	//println!("Response Bytes: {:?}", response_bytes);
	let response_string = jsonxf::pretty_print(&response_string).unwrap();

	std::fs::write("target/response_check.json", &response_string)?;
	Ok(serde_json::from_str(&response_string)?)
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
		match query_search(input) {
			Ok(response) => {
				let mut content = String::with_capacity(1000);
				for item in response.get_items() {
					item.display(&mut content, false);
				}
				result_text.set_content(content);
			}
			Err(err) => {
				let content = format!("Error: {:?}", err);
				result_text.set_content(content);
			}
		}
		//s.add_layer(Dialog::info(format!("Searching: {}", input)));
	};
}

use std::{fs::File, io::BufReader, iter::Iterator};

use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiMatchQuery {
	_name: String,
	analyzer: String,
	auto_generate_synonyms_phrase_query: bool,
	fields: Value,
	operator: String,
	query: String,
	#[serde(rename = "type")]
	query_type: String,
}
impl MultiMatchQuery {
	pub fn template<'a>(&self, search_input: &'a str) -> impl Iterator<Item = Self> + 'a {
		let mut template = self.clone();
		let search_input_underscored = search_input.replace(" ", "_").to_lowercase();
		template._name = format!("multi_match_{}", search_input_underscored);
		template.query = search_input.to_owned();
		Some(template).into_iter()
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PackageAttrName {
	value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WildcardQuery {
	package_attr_name: PackageAttrName,
}
impl WildcardQuery {
	pub fn template<'a>(&self, search_input: &'a str) -> impl Iterator<Item = Self> + 'a {
		let template = self.clone();
		search_input.split_whitespace().map(move |input_word|{
			let mut query = template.clone();
			query.package_attr_name.value = format!("*{}*", input_word.to_lowercase());
			query
		})
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum RequestQueryEnum {
	#[serde(rename="multi_match")]
	MultiMatch(MultiMatchQuery),
	#[serde(rename="wildcard")]
	Wildcard(WildcardQuery),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DisMax {
	tie_breaker: f32,
	queries: Vec<RequestQueryEnum>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MustType {
	dis_max: DisMax,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Bool {
	filter: Value,
	must: [MustType; 1],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RequestQuery {
	bool: Bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
	aggregations: Value,
	from: i32,
	query: RequestQuery,
	size: i32,
	sort: Value,
}

pub struct RequestTemplate {
	request_template: Request,
	multi_match_template: MultiMatchQuery,
	wildcard_template: WildcardQuery,
}
impl RequestTemplate {
	pub fn new(request_template_path: &str, multi_match_template_path: &str, wildcard_template_path: &str) -> anyhow::Result<Self> {
		Ok(Self {
			request_template: serde_json::from_reader(BufReader::new(File::open(request_template_path)?))?,
			multi_match_template: serde_json::from_reader(BufReader::new(File::open(multi_match_template_path)?))?,
			wildcard_template: serde_json::from_reader(BufReader::new(File::open(wildcard_template_path)?))?
		})
	}
}
impl RequestTemplate {
	pub fn template(&self, search_input: &str) -> Request {

		let templated_wildcards = self.wildcard_template.template(search_input).map(|q|RequestQueryEnum::Wildcard(q));
		let templated_multi_matches = self.multi_match_template.template(search_input).map(|q|RequestQueryEnum::MultiMatch(q));
		let queries = templated_multi_matches.chain(templated_wildcards);
		
		let mut request = self.request_template.clone();
		request.query.bool.must[0].dis_max.queries = queries.collect();
		request
	}
}
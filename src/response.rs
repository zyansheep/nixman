#![allow(non_snake_case)]

use serde::de::IgnoredAny;

/* #[derive(Debug, Deserialize)]
struct PackageResponse {
	
} */
#[derive(Debug, Deserialize)]
struct SortResponse(f32, String, String);

#[derive(Debug, Deserialize)]
struct HitSource {
	#[serde(rename = "type")]
	package_type: String,
	package_hydra: IgnoredAny,
	package_attr_name: String,
	package_attr_name_reverse: Option<IgnoredAny>,
	package_attr_name_query: IgnoredAny, // ["r", "rPackages", "rPackages.hello", "rPackages.helloJava", "rPackages.helloJavaWorld", "Packages", "Packages.hello", "Packages.helloJava", "Packages.helloJavaWorld", "hello", "helloJava", "helloJavaWorld", "Java", "JavaWorld", "World"],
	package_attr_name_query_reverse: Option<IgnoredAny>, // ["r", "segakcaPr", "olleh.segakcaPr", "avaJolleh.segakcaPr", "dlroWavaJolleh.segakcaPr", "segakcaP", "olleh.segakcaP", "avaJolleh.segakcaP", "dlroWavaJolleh.segakcaP", "olleh", "avaJolleh", "dlroWavaJolleh", "avaJ", "dlroWavaJ", "dlroW"],
	package_attr_set: Option<String>,
	package_attr_set_reverse: Option<IgnoredAny>, // "segakcaPr",
	package_pname: String,
	package_pname_reverse: Option<IgnoredAny>, // "dlroWavaJolleh-r",
	package_pversion: String,
	package_description: Option<String>,
	package_description_reverse: Option<IgnoredAny>, // null,
	package_longDescription: Option<String>,
	package_longDescription_reverse: Option<IgnoredAny>, // "",
	package_license: IgnoredAny, /* [{
		fullName: "No license",
		url: null
	}], */
	package_license_set: Option<Vec<String>>,
	package_maintainers: IgnoredAny, /* [{
		name: "No maintainers",
		email: null,
		github: null
	}], */
	package_maintainers_set: Option<Vec<String>>,
	package_platforms: Vec<String>, // ["aarch64-linux", "i686-linux", "x86_64-linux", "x86_64-darwin"],
	package_position: String,
	package_homepage: Option<String>,
	package_system: IgnoredAny,
}

#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
struct Hit {
	_index: IgnoredAny,
	_type: IgnoredAny,
	_id: IgnoredAny,
	_score: f32,
	_source: HitSource,
	sort: SortResponse,
	#[derivative(Debug="ignore")]
	matched_queries: IgnoredAny,
}

#[derive(Debug, Deserialize)]
struct Hits {
	total: serde_json::Value,
    max_score: serde_json::Value,
	pub hits: Vec<Hit>,
}

#[derive(Debug)]
pub struct ResponseItem {
	pub name: String,
	pub version: String,
	pub platforms: Vec<String>,
}

impl ResponseItem {
	pub fn display(&self, buf: &mut String, _selected: bool) {
		buf.push_str(&format!(
r#"{} - {}
"#, self.name, self.version
		))
	}
}

#[derive(Debug, Deserialize)]
pub struct Response {
	took: i32,
	timed_out: bool,
	_shards: IgnoredAny,
	hits: Hits,
	//aggregations: IgnoredAny,
}
impl Response {
	pub fn get_items(mut self) -> impl Iterator<Item = ResponseItem> {
		
		/* self.hits.hits.dedup_by(|hit1, hit2|{
			hit1._source.package_attr_name == hit2._source.package_attr_name &&
			hit1._source.package_pversion == hit2._source.package_pversion
		}); */
		self.hits.hits.into_iter().map(|hit|{
			ResponseItem {
				name: hit._source.package_attr_name,
				version: hit._source.package_pversion,
				platforms: hit._source.package_platforms,
			}
		})
	}
}
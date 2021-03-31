use serde::de::IgnoredAny;


/* #[derive(Debug, Deserialize)]
struct PackageResponse {
	
} */
#[derive(Debug, Deserialize)]
struct SortResponse(f32, String, String);

#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
struct Hit {
	_index: IgnoredAny,
	_type: IgnoredAny,
	_id: IgnoredAny,
	_score: f32,
	#[derivative(Debug="ignore")]
	_source: IgnoredAny,
	sort: SortResponse,
	#[derivative(Debug="ignore")]
	matched_queries: IgnoredAny,
}

#[derive(Debug, Deserialize)]
struct Hits {
	total: serde_json::Value,
    max_score: serde_json::Value,
	hits: Vec<Hit>,
}

#[derive(Debug, Deserialize)]
pub struct Response {
	took: i32,
	timed_out: bool,
	_shards: IgnoredAny,
	hits: Hits,
	aggregations: IgnoredAny,
}
pub struct World {
    summaries: Vec<PlaceSummary>,
}

struct PlaceSummary {
    id: String,
    geo: (f32, f32),
    url: String,
    title: String,
    country: String,
}

pub struct Place {
    title: String,
    subtitle: String,
    url: String,
    map: String,
    count: u16,
    utc_offset: i32,
    stations: Vec<StationSummary>,
}

pub struct StationSummary {
    title: String,
    href: String,
}

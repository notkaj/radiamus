use radiobrowser::ApiCountry;

pub struct Country {
    name: String,
    code: String,
    station_count: u16,
}

impl From<ApiCountry> for Country {
    fn from(value: ApiCountry) -> Self {
        Self {
            name: value.name,
            code: value.iso_3166_1,
            station_count: value.stationcount,
        }
    }
}

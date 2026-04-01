use radiobrowser::{self, ApiCountry, ApiStation, CountryOrder, RadioBrowserAPI};
use std::error::Error;

pub struct ApiContext {
    api: RadioBrowserAPI,
}

impl ApiContext {
    async fn new() -> Result<Self, Box<dyn Error>> {
        let api = RadioBrowserAPI::new().await?;
        Ok(Self { api })
    }
    async fn stations_by_name(&mut self, name: String) -> Result<Vec<ApiStation>, Box<dyn Error>> {
        let stations = self
            .api
            .get_stations()
            .name(name)
            .order(radiobrowser::StationOrder::Clickcount)
            .send()
            .await?;

        Ok(stations)
    }

    pub async fn countries(&self) -> Result<Vec<ApiCountry>, Box<dyn Error>> {
        self.api
            .get_countries()
            .order(CountryOrder::StationCount)
            .send()
            .await
    }
}

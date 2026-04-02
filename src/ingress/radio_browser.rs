use color_eyre::Result;
use color_eyre::eyre::eyre;
use radiobrowser::{self, ApiCountry, ApiStation, CountryOrder, RadioBrowserAPI};

static CONTEXT: Context = Context::new();

pub fn context() -> &'static Context {
    &CONTEXT
}

#[derive(Default)]
pub struct Context {
    api: Option<RadioBrowserAPI>,
}

impl Context {
    pub const fn new() -> Self {
        Self { api: None }
    }

    pub async fn init(&mut self) -> Result<()> {
        let api = RadioBrowserAPI::new().await?;
        self.api = Some(api);
        Ok(())
    }

    #[allow(dead_code)]
    async fn stations_by_name(&self, name: String) -> Result<Vec<ApiStation>> {
        let Some(api) = &self.api else {
            return Err(eyre!(
                "Coult not retrieve stations: ApiContext not initialized"
            ));
        };

        Ok(api
            .get_stations()
            .name(name)
            .order(radiobrowser::StationOrder::Clickcount)
            .send()
            .await?)
    }

    pub async fn countries(&self) -> Result<Vec<ApiCountry>> {
        let api = self.api()?;

        Ok(api.get_countries().send().await?)
    }

    pub async fn countries_by_order(&self, order: CountryOrder) -> Result<Vec<ApiCountry>> {
        Ok(self.api()?.get_countries().order(order).send().await?)
    }

    fn api(&self) -> Result<&RadioBrowserAPI> {
        let Some(api) = &self.api else {
            return Err(eyre!(
                "Coult not retrieve stations: ApiContext not initialized"
            ));
        };
        Ok(api)
    }
}

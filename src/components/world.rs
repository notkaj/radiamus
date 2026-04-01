use crate::components::country::Country;
use crate::ingress::radio_browser::ApiContext;

#[derive(Default)]
pub struct World {
    countries: Vec<Country>,
}

impl World {
    async fn populate(&mut self, context: &ApiContext) -> Result<()> {
        self.countries = context.countries().await?;
        Ok(())
    }
}

impl Component for World {}

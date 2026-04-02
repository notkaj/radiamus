use std::error::Error;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::List;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::components::Component;
use crate::components::country::Country;
use crate::ingress::radio_browser::ApiContext;
use radiobrowser::ApiCountry;

pub struct World {
    countries: Option<Vec<Country>>,
    tx: Sender<Vec<Country>>,
    rx: Receiver<Vec<Country>>,
}

impl World {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(1);
        Self {
            countries: None,
            tx,
            rx,
        }
    }

    pub async fn init(&self, context: &ApiContext) {
        let tx = self.tx.clone();
        tokio::spawn(Self::populate(&context, tx));
    }

    pub async fn populate(context: &ApiContext, tx: Sender<Vec<Country>>) {
        let api_countries = context.countries().await.unwrap();
        let countries = api_countries.into_iter().map(|c| c.into()).collect();
        tx.send(countries);
    }
}
impl Component for World {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let items = self.countries.iter().map(|c| c.name.to_owned());
        let list = List::new(items);
        frame.render_widget(list, area);

        Ok(())
    }
}

impl From<ApiCountry> for Country {
    fn from(value: ApiCountry) -> Self {
        Self {
            name: value.name,
            code: value.iso_3166_1,
            station_count: value.stationcount as u16,
        }
    }
}

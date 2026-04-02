use color_eyre::eyre::eyre;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::List;
use throbber_widgets_tui::Throbber;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::action::Action;
use crate::components::Component;
use crate::components::country::Country;
use crate::ingress::radio_browser::context;
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

    pub async fn connect(&self) {
        let tx = self.tx.clone();
        tokio::spawn(Self::populate(tx));
    }

    pub async fn refresh(&mut self) {
        self.countries = None;
        // let tx = self.tx.clone();
        // tokio::spawn(Self::populate(tx));
        self.connect().await;
    }

    async fn populate(tx: Sender<Vec<Country>>) {
        let api_countries = context().countries().await.unwrap();
        let countries = api_countries.into_iter().map(|c| c.into()).collect();
        let _ = tx.send(countries).await;
    }
}
impl Component for World {
    fn update(
        &mut self,
        action: crate::action::Action,
    ) -> color_eyre::Result<Option<crate::action::Action>> {
        match action {
            Action::Tick => {
                if let Ok(c) = self.rx.try_recv() {
                    self.countries = Some(c);
                }
            }
            _ => return Ok(None),
        }

        Ok(None)
    }
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        match &self.countries {
            Some(c) => {
                let items = c.iter().map(|c| c.name.to_owned());
                let list = List::new(items);
                frame.render_widget(list, area);
            }
            None => {
                // return throbber widget
                let throbber = Throbber::default().label("Loading...");
                frame.render_widget(throbber, area);
            }
        };
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

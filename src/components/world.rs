use color_eyre::Result;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::List;
use throbber_widgets_tui::Throbber;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::action::Action;
use crate::components::country::Country;
use crate::components::{Component, Populatable};
use crate::ingress::radio_browser::CONTEXT;
use radiobrowser::ApiCountry;

pub struct World {
    countries: Option<Vec<Country>>,
    tx: Sender<Result<Vec<Country>>>,
    rx: Receiver<Result<Vec<Country>>>,
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

    async fn retreive(tx: Sender<Result<Vec<Country>>>) {
        let guard = CONTEXT.read().await;
        let _ = match guard.countries().await {
            Ok(c) => {
                let countries = c.into_iter().map(|c| c.into()).collect();
                tx.send(Ok(countries)).await
            }
            Err(e) => tx.send(Err(e)).await,
        };
    }
}

impl Populatable for World {
    fn populate(&mut self) -> Result<()> {
        self.countries = None;
        let tx = self.tx.clone();
        tokio::spawn(Self::retreive(tx));
        Ok(())
    }

    fn refresh(&mut self) -> Result<()> {
        self.countries = None;
        self.populate()
    }
}

impl Component for World {
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
        match key.code {
            crossterm::event::KeyCode::Char('r') => self.refresh()?,
            crossterm::event::KeyCode::Right => (),
            _ => (),
        }

        Ok(None)
    }
    fn update(
        &mut self,
        action: crate::action::Action,
    ) -> color_eyre::Result<Option<crate::action::Action>> {
        match action {
            Action::Tick => {
                if self.countries.is_some() {
                    return Ok(None);
                }
                if let Ok(r) = self.rx.try_recv() {
                    match r {
                        Ok(c) => {
                            self.countries = Some(c);
                        }
                        Err(e) => return Err(e),
                    }
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

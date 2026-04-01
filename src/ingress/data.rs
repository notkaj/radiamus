mod radio_browser_api;
// use serde::Deserialize;
//
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct Places {
//     data: PlacesData,
// }
//
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct PlacesData {
//     list: Vec<PlaceSummary>,
//     version: String,
// }
//
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct PlaceSummary {
//     id: String,
//     geo: Vec<f32>,
//     url: String,
//     size: u16,
//     boost: bool,
//     title: String,
//     country: String,
// }
//
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct Page {
//     data: PageData,
// }
//
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct PageData {
//     title: String,
//     subtitle: String,
//     url: String,
//     map: String,
//     count: u16,
//     utc_offset: i32,
//     content: PageContent,
// }
//
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct Channels {
//     data: ChannelsData,
// }
//
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct ChannelsData {
//     title: String,
//     subtitle: String,
//     url: String,
//     map: String,
//     count: u16,
//     utc_offset: i32,
//     content: Vec<ChannelsContent>,
// }
//
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct ChannelsContent {
//     items: Vec<ChannelSummary>,
// }
//
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct ChannelSummary {
//     title: String,
//     href: String,
// }

#![allow(non_snake_case)]

use crate::api::http::Context;
use crate::top::interface::{WallhavenResponse, WallhavenResult};
use crate::utils::error::{Error, WallResult};
use scraper::Selector;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopTag {
    pub categories: i64,
    pub purity: i64,
    pub topRange: String,
    pub sorting: String,
    pub order: String,
    pub ai_art_filter: i64,
    pub page: i64,
}



impl TopTag {
    pub fn new(
        self
    ) -> Self {
        self
    }
    pub fn get_url(&self) -> String {
        if self.topRange.is_empty() {
            format!(
                "https://wallhaven.cc/search?categories={}&purity={}&sorting={}&order={}&ai_art_filter={}&page={}",
                self.categories, self.purity, self.sorting, self.order, self.ai_art_filter, self.page
            )
        } else {
            format!(
                "https://wallhaven.cc/search?categories={}&purity={}&topRange={}&sorting={}&order={}&ai_art_filter={}&page={}",
                self.categories, self.purity, self.topRange, self.sorting, self.order, self.ai_art_filter, self.page
            )
        }
        // format!("https://wallhaven.cc/search?categories={}&purity={}&topRange={}&sorting={}&order={}&ai_art_filter={}&page={}", self.categories, self.purity, self.topRange, self.sorting, self.order, self.ai_art_filter, self.page)
    }

    pub async fn get_top_page(&self, context: State<'_, Context>) -> WallResult<WallhavenResult> {
        let url = self.get_url();
        let client = context.http_client();
        let response = client.get(url).send().await?;
        let body = response.text().await?;
        let document = scraper::Html::parse_document(&body);
        let main_selector = Selector::parse("main")?;
        let section_selector = Selector::parse("section")?;
        let ul_selector = Selector::parse("ul")?;
        let li_selector = Selector::parse("li")?;
        let main = document
            .select(&main_selector)
            .next()
            .ok_or(Error::new("Main not found"))?;
        let section = main
            .select(&section_selector)
            .next()
            .ok_or(Error::new("Section not found"))?;
        let ul = section
            .select(&ul_selector)
            .next()
            .ok_or(Error::new("Ul not found"))?;
        let mut response = WallhavenResult::new();
        for element in ul.select(&li_selector) {
            let img_selector = Selector::parse("img[data-src]")?;
            let img = element
                .select(&img_selector)
                .next()
                .ok_or(Error::new("Image not found"))?;
            let src = img
                .value()
                .attr("data-src")
                .ok_or(Error::new("Src not found"))?;
            let png_value = Selector::parse("span.png")?;
            let png = element.select(&png_value).next();
            let wall_res = Selector::parse("span.wall-res")?;
            let width_height_html = element
                .select(&wall_res)
                .next()
                .ok_or(Error::new("Wall res not found"))?;
            let width_height_collect = width_height_html.text().collect::<Vec<_>>();
            let width_height = width_height_collect[0]
                .split(" x ")
                .map(|x| {
                    x.parse::<i32>().unwrap_or_else(|_| {
                        panic!(
                            "{}",
                            Error::new("Failed to parse width and height").to_string()
                        )
                    })
                })
                .collect::<Vec<i32>>();

            let b_src = src.replace("th.wallhaven.cc/small", "w.wallhaven.cc/full");
            let parts: Vec<&str> = b_src.rsplitn(3, '/').collect();
            if parts.len() != 3 {
                println!("Invalid URL");
                continue;
            }
            let file_name = parts[0];
            let tag = parts[1];
            let new_file_name = format!("wallhaven-{}", file_name);

            if png.is_some() {
                let _new_file_name: Vec<&str> = new_file_name.rsplitn(2, '.').collect();
                let new_file_name = format!("{}.png", _new_file_name[1]);
                let new_url = format!("https://w.wallhaven.cc/full/{}/{}", tag, new_file_name);
                response.push(WallhavenResponse::new(
                    new_file_name.to_string(),
                    src.to_string(),
                    new_url.to_string(),
                    width_height[0],
                    width_height[1],
                ));
            } else {
                let new_url = format!("https://w.wallhaven.cc/full/{}/{}", tag, new_file_name);
                response.push(WallhavenResponse::new(
                    new_file_name.to_string(),
                    src.to_string(),
                    new_url.to_string(),
                    width_height[0],
                    width_height[1],
                ));
            }
        }
        Ok(response)
    }
}

use scraper::Selector;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::api::http::Context;
use crate::tags::interface::{TagsBase, TagsResult};
use crate::utils::error::{Error, WallResult};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tags {
    pub page: i64,
    pub categories: Option<String>,
}


impl Tags {
    pub fn new(
        self
    ) -> Self {
        self
    }

    pub fn get_url(&self) -> String {
        match &self.categories {
            Some(categories) => {
                format!(
                    "https://wallhaven.cc/tags/{}?page={}",
                    categories, self.page
                )
            },
            None => {
                format!(
                    "https://wallhaven.cc/tags?page={}",
                    self.page
                )
            }
        }
    }

    pub async fn get_content(&self, context: State<'_, Context>) -> WallResult<TagsResult> {
        let url = self.get_url();
        let client = context.http_client();
        let response = client.get(url).send().await?;
        let body = response.text().await?;
        let document = scraper::Html::parse_document(&body);
        let main_selector = Selector::parse("main")?;
        let section_selector = Selector::parse("section.col-nav-subsection")?;
        let div_tag_list = Selector::parse("div.grid")?;
        let tag_list_tag_main = Selector::parse("div.taglist-tagmain")?;
        let main = document
            .select(&main_selector)
            .next()
            .ok_or(Error::new("Main not found"))?;
        let section = main
            .select(&section_selector)
            .next()
            .ok_or(Error::new("Section not found"))?;
        let div_tag_list = section
            .select(&div_tag_list)
            .next()
            .ok_or(Error::new("div tag list not found"))?;

        let tag_info = Selector::parse("a[href]")?;
        let mut response = TagsResult::new();
        for element in div_tag_list.select(&tag_list_tag_main) {
            let info_url = element
                .select(&tag_info)
                .next()
                .ok_or(Error::new("tag info not found"))?;
            let url = info_url.value().attr("href").ok_or(Error::new("tag info url not found"))?;
            let name = info_url.text().collect::<Vec<_>>().join(" ");
            response.push(TagsBase {
                name,
                url: url.to_string(),
            });

        }
        Ok( response )
    }

}

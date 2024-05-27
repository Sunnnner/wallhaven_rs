

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TagsBase {
    pub name: String,
    pub url: String,
}

pub type TagsResult = Vec<TagsBase>;
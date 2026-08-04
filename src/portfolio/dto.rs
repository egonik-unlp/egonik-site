#[derive(Debug, Clone)]
pub struct TagDto {
    pub id: i32,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct PortfolioItemDto {
    pub title: String,
    pub description: String,
    pub public: bool,
    pub public_url: Option<String>,
    pub tags: Vec<TagDto>,
}

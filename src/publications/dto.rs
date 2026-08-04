#[derive(Debug, Clone)]
pub struct PublicationItemDto {
    pub title: String,
    pub abs: String,
    pub year: i32,
    pub journal: String,
    pub link: String,
}

impl PublicationItemDto {
    pub fn new(title: String, abs: String, year: i32, journal: String, link: String) -> Self {
        Self {
            title,
            abs,
            year,
            journal,
            link,
        }
    }
}

pub mod config;
pub trait Repository<T, U> {
    fn get_all(&mut self) -> anyhow::Result<Vec<T>>;
    fn get_by_id(&mut self, id: i32) -> anyhow::Result<T>;
    fn create_article(&mut self, article: U) -> anyhow::Result<()>;
}

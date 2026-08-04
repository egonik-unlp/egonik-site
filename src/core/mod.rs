pub trait Repository<T> {
    fn get_all(&mut self) -> anyhow::Result<Vec<T>>;
    fn get_by_id(&mut self, id: i32) -> anyhow::Result<T>;
}

pub mod stats;
pub mod files;

pub trait ApiRoute<T> {
    fn get_route(&self) -> &str;
}

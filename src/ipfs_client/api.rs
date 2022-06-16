pub mod stats;

pub trait ApiRoute<T> {
    fn get_route(&self) -> &str;
}

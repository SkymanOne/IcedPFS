pub mod bandwidth;

pub trait ApiRoute<T> {
    fn get_route(&self) -> String;
}
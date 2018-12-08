mod request;
mod response;

pub use request::{Request};
pub use response::{Response};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

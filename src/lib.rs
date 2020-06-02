mod pswdmng;

pub use crate::pswdmng::cli;
pub use crate::pswdmng::error::Error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

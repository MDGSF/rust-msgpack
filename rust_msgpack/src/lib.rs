pub mod binary;
pub mod bytes;
pub mod codes;
pub mod decode;
pub mod encode;
pub mod error;
pub mod ext;
pub mod time;
pub mod transvalue;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

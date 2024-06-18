#![feature(coroutine_trait)]
#![feature(coroutines)]
extern crate core;

mod compiler;
pub mod v1;
mod events;
mod pattern;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

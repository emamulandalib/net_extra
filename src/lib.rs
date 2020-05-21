mod ip;

pub use ip::{IpMask};


#[cfg(test)]
mod tests {
    use super::ip::*;
    use std::collections::HashMap;

    #[test]
    fn all_values_are_zero_should_return_true() {
        assert_eq!(is_zeros(&[0, 0, 0, 0]), true)
    }

    #[test]
    fn all_values_are_not_zero_should_return_false() {
        assert_eq!(is_zeros(&[0, 0, 12, 0]), false)
    }
}
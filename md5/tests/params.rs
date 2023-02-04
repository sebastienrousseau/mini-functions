#[cfg(test)]

// TODO: Add more tests to bring the code coverage to 100%
mod tests {
    extern crate md5;
    use md5::*;

    #[test]
    fn test_md5_params_new() {
        let params = MD5Params::new(0, 1, 2, 3, 4, 5, 6);
        assert_eq!(params.a, 0);
        assert_eq!(params.b, 1);
        assert_eq!(params.c, 2);
        assert_eq!(params.d, 3);
        assert_eq!(params.x, 4);
        assert_eq!(params.s, 5);
    }

    #[test]
    fn test_f() {
        let mut params: MD5Params = MD5Params::new(0, 1, 2, 3, 4, 5, 6);
        assert_eq!(f(&mut params), ((!1 & 3) + 1));
    }

    #[test]
    fn test_g() {
        let mut params = MD5Params::new(0, 1, 2, 3, 4, 5, 6);
        assert_eq!(g(&mut params), 1 & 3);
    }

    #[test]
    fn test_h() {
        let mut params = MD5Params::new(0, 1, 2, 3, 4, 5, 6);
        assert_eq!(h(&mut params), 1 ^ 2 ^ (3 - 1));
    }

    #[test]
    fn test_i() {
        let mut params = MD5Params::new(0, 1, 2, 3, 4, 5, 6);
        assert_eq!(i(&mut params), 2 ^ ((1 | !3) - 1));
    }
    #[test]
    fn test_md5_f() {
        assert_eq!(md5_f(1, 2, 3), (!1 & 3));
    }

    #[test]
    fn test_md5_h() {
        assert_eq!(md5_h(1, 2, 3), 1 ^ 2 ^ 3);
    }

    #[test]
    fn test_md5_i() {
        assert_eq!(md5_i(1, 2, 3), 2 ^ (1 | !3));
    }

    #[test]
    fn test_rotate_left() {
        assert_eq!(rotate_left(0b1010, 2), 0b101000);
    }    
}

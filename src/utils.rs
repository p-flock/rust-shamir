#![allow(non_snake_case)]
pub mod utilities {

    pub fn modulo(a: i64, b: i64) -> i64 {
        ((a % b) + b) % b
    }
    #[test]
    fn test_modulo() {
        assert_eq!(modulo(3, 2), 1);
        assert_eq!(modulo(-1, 65413), 65412);
    }

    /// modular exponentiation
    pub fn mod_exp(base: i64, exp: i64, modulus: i64) -> i64 {
        let mut b = base;
        let mut e = exp;
        let mut result = 1;
        while e > 0 {
            if e % 2 == 0 {}
            else {
                result = modulo(result * b, modulus);
            }
            b = modulo(b * b, modulus);
            e /= 2; //
        }
        result
    }

    #[test]
    fn test_modular_exp() {
        let p = crate::P;
        assert_eq!(mod_exp(2, 0, p), (1));
        assert_eq!(mod_exp(2, 4, p), (16));
        assert_eq!(mod_exp(2, 6, 31), (2));
        assert_eq!(mod_exp(3, 4, 31), (19));
    }

    // Adapted from:
    //      https://brilliant.org/wiki/extended-euclidean-algorithm/
    //      https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
    pub fn egcd(b: i64, n: i64) -> (i64, i64, i64) {
        // the multiplicative inverse of b mod n is the second value returned in the tuple
        if b == 0 {
            (n, 0, 1)
        } else {
            let (gcd, x, y) = egcd(modulo(n, b), b);
            (gcd, y - (n/b) * x, x)
        }
    }
    #[test]
    fn test_egcd() {
        assert_eq!(egcd(30, 50), (10, 2, -1));
        assert_eq!(egcd(12, 8), (4, 1, -1));
        assert_eq!(egcd(4, 11), (1, 3, -1));
        assert_eq!(egcd(1, 1), (1, 1, 0));
    }
    pub fn mod_inverse(base: i64, modulus: i64) -> i64 {
        egcd(base, modulus).1
    }
    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(4, 11), 3);
    }
    /// performs lagrange interpolation on the given points at point zero
    /// adapted from: https://www.codewithc.com/c-program-for-lagrange-interpolation/
    ///               https://en.wikipedia.org/wiki/Lagrange_polynomial
    pub fn interpolate_at_zero(x_values: Vec<i64>, y_values: Vec<i64>, degree: i64) -> i64 {
        assert_eq!(x_values.len(), y_values.len());
        let n = x_values.len();
        assert!(degree + 1 <= n as i64, "Polynomial of degree {} cannot be interpolated accurately with only {} points", degree, n);

        let P = crate::P;
        let a = 0;
        let mut k = 0;
        for i in 0..n {
            let mut s = 1;
            let mut t = 1;
            for j in 0..n {
                if j != i {
                    s = s * (a - x_values[j]);
                    s = modulo(s,P);
                    t = t * (x_values[i] - x_values[j]);
                    t = modulo(t,P);
                }
            }
            k = k + ((s * mod_inverse(t, P)) * y_values[i]);
            k = modulo(k, P);
        }
        k
    }
    #[test]
    fn test_interpolate() {
        assert_eq!(interpolate_at_zero(vec![1, 2], vec![2, 4], 1), 0);
        assert_eq!(interpolate_at_zero(vec![1, 2, 3], vec![3, 6, 11], 2), 2);
    }
}

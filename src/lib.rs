use rand::{thread_rng, Rng};
mod utils;
use crate::utils::{utilities};

pub const P: i64  = 65413; // 16 bit prime,

#[derive(Debug)]
pub struct Share {
    x: i64,
    y: i64,
    threshold: i64,
    id: String
}

#[derive(Debug)]
struct Polynomial {
    degree: i64,
    coefficients: Vec<i64>, // coefficients[0] is the y_intercept (b in, y=mx+b) as it is the coefficient for the 0th term
}

impl Polynomial {
    pub fn new_random_poly(y_intercept: i64, degree: i64) -> Polynomial {
        let mut coefficients: Vec<i64>  = Vec::new();
        coefficients.push(y_intercept);
        let mut rng = thread_rng();
        for _ in 0..degree {
            let a: i64 = rng.gen_range(0, P);
            coefficients.push(a);
        }
        Polynomial {
            degree: degree,
            coefficients: coefficients,
        }
    }
    // only used if you want to specify the polynomial as opposed to creating a random one
    // create_shares uses new_random_poly
    pub fn new(coefficients: Vec<i64>) -> Polynomial {
        let degree: i64 = (coefficients.len() - 1) as i64;
        Polynomial {
            degree: degree,
            coefficients: coefficients

        }
    }
    pub fn eval_at_point(&self, point: i64) -> i64 {
        let mut result: i64 = 0;
        for (degree, coeff) in self.coefficients.iter().enumerate() {
            let d = degree as i64;
            result += (coeff * utilities::mod_exp(point, d, P)) % P;
        }
        result % P
    }

}
#[test]
fn test_polynomial_impl() {
    assert_eq!(0, 0);
}

/// Split a secret into n shares via shamir
/// returns a vector of share objects which each represent a distinct point on a random polynomial
/// with root = secret
pub fn create_shares(secret: i64, n: i64, threshold: i64) -> Vec<Share> {
    let poly = Polynomial::new_random_poly(secret, threshold - 1);
    //println!("{:?}", poly);
    let mut shares: Vec<Share> = Vec::new();
    for x in 1..n+1 { // don't eval at 0 (this is the secret)
        shares.push(Share{
            x: x,
            y: poly.eval_at_point(x),
            threshold,
            id: String::from("")
        });
    }
    shares
}

/// reconstruct a secret based on a set of shares (x, y) coordinates
/// using lagrange interpolation
pub fn reconstruct(shares: &Vec<Share>) -> i64 {
    let t = shares[0].threshold;
    assert!(shares.len() as i64 >= t, "not enough shares to reconstruct secret with threshold {}.", t);

    let x_values = shares.iter().map(|share| share.x).collect::<Vec<i64>>();
    let y_values = shares.iter().map(|share| share.y).collect::<Vec<i64>>();
    utilities::interpolate_at_zero(x_values, y_values, t - 1) % P
}
#[test]
fn test_share_and_reconstruct() {
    let mut secret = 100;
    let mut shares = create_shares(secret, 3, 3);
    let mut recons = reconstruct(&shares);
    assert_eq!(secret, recons);

    secret = 60;
    shares = create_shares(secret, 8, 4);
    println!("{:?}", shares);
    recons = reconstruct(&shares);
    assert_eq!(secret, recons);

    let mut rng = thread_rng();
    let a: i64 = rng.gen_range(0, P);
    shares = create_shares(a, 10, 10);
    recons = reconstruct(&shares);
    assert_eq!(a, recons);
}

/// Adds two vectors of shares, representing polynomials
pub fn add_shares(a: &Vec<Share>, b: &Vec<Share>) -> Vec<Share> {
    assert_eq!(a.len(), b.len(), "Can only add shares with equal number of points, otherwise may be unable to reconstruct");

    a.iter().zip(b.iter()).map(|tup| tup.0.add(tup.1)).collect::<Vec<Share>>()
}
#[test]
fn test_add_shares() {
    let mut rng = thread_rng();
    let a: i64 = rng.gen_range(0, P);
    let b: i64 = rng.gen_range(0, P);
    let s1 = create_shares(a, 5, 5);
    let s2 = create_shares(b, 5, 5);

    let result = utilities::modulo(a + b, P);
    let c = add_shares(&s1, &s2);
    let recons = reconstruct(&c);
    assert_eq!(result, recons);
}


impl Share {
    pub fn add(&self, o: &Share) -> Share {
        assert!(self.threshold == o.threshold, "Shares do not have the same threshold");
        assert!(self.x == o.x, "Cannot add two shares with different x-coordinates");

        Share {
            x: self.x,
            y: self.y + o.y,
            threshold: self.threshold,
            id: String::from("")
        }
    }
}

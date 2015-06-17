extern crate num;
extern crate nalgebra;

use std::f64;
use nalgebra::Vec2;
use nalgebra::Norm;
use num::traits::Zero;

// const RT1A : f64 = 0.625f64.sqrt();
const RT1A : f64 = 0.7905694150420949f64;
// const RT1B : f64 = 0.125f64.sqrt();
const RT1B : f64 = 0.3535533905932738f64;
// const RT2 : f64 = 0.125f64.sqrt();
const RT2 : f64 = 0.3535533905932738f64;
const P1RHO : f64 = 0.6f64;
const P2RHO : f64 = 0.1f64;

fn calc_r1vec(x : f64, y : f64) -> Vec2<f64> {
    Vec2::new(x - 1.25f64, y - 0.75f64)
}
fn calc_r2vec(x : f64, y : f64) -> Vec2<f64> {
    let z : f64 = (0.5f64 + x - y) * 0.5f64;
    Vec2::new(z, -z)
}

fn u_laplace(x : f64, y : f64) -> f64 {
    let r1 = calc_r1vec(x, y).norm();
    let r2 = calc_r2vec(x, y).norm();
    let term1 =
        if RT1B <= r1 && r1 < RT1A {
            P1RHO
        } else {
            0.0
        };
    let term2 =
        if r2 < RT2 {
            P2RHO
        } else {
            0.0
        };
    term1 + term2
}

fn u_grad(x : f64, y : f64) -> Vec2<f64> {
    let r1vec = calc_r1vec(x, y);
    let r2vec = calc_r2vec(x, y);
    let r1 = r1vec.norm();
    let r2 = r2vec.norm();
    let term1a : f64 =
        if r1 < RT1A {
            0.5f64
        } else {
            (RT1A / r1).powi(2) * 0.5f64
        };
    let term1b : f64 =
        if r1 < RT1B {
            0.5f64
        } else {
            (RT1B / r1).powi(2) * 0.5f64
        };
    let term1 = (term1a - term1b) * P1RHO;
    let term2 : f64 =
        if r2 < RT2 {
            P2RHO
        } else {
            P2RHO / r2
        };
    r1vec * term1 + r2vec * term2
}
fn u(x : f64, y : f64) -> f64 {
    let r1 = calc_r1vec(x, y).norm();
    let r2 = calc_r2vec(x, y).norm();
    let term1a : f64 =
        if r1 < RT1A {
            P1RHO * 0.25f64 * r1.powi(2)
        } else {
            P1RHO * 0.5f64 * RT1A.powi(2) * (0.5f64 + f64::ln(r1/RT1A))
        };
    let term1b : f64 =
        if r1 < RT1B {
            P1RHO * 0.25f64 * r1.powi(2)
        } else {
            P1RHO * 0.5f64 * RT1B.powi(2) * (0.5f64 + f64::ln(r1/RT1B))
        };
    let term1 : f64 = term1a - term1b;
    let term2 : f64 =
        if r2 < RT2 {
            P2RHO * 0.5f64 * r2.powi(2)
        } else {
            P2RHO * RT2 * (r2 - 0.5f64 * RT2)
        };
    term1 + term2
}

fn main() {
    println!("Hello, world!");
}

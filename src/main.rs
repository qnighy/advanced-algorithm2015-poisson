extern crate num;
extern crate nalgebra;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::f64;
use nalgebra::Vec2;
use nalgebra::Norm;

const EPS : f64 = 1e-12f64;

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

fn vecdot(x : &Vec<f64>, y : &Vec<f64>) -> f64 {
    let mut sum : f64 = 0.0f64;
    for (xi, yi) in x.iter().zip(y.iter()) {
        sum += xi * yi;
    }
    sum
}
fn vecpluseq_alpha(x : &mut Vec<f64>, alpha : f64, y : &Vec<f64>) {
    for (xi, yi) in x.iter_mut().zip(y.iter()) {
        *xi += alpha * yi;
    }
}
fn vecminus3(x : &Vec<f64>, y : &Vec<f64>, z : &mut Vec<f64>) {
    for ((xi, yi), zi) in x.iter().zip(y.iter()).zip(z.iter_mut()) {
        *zi = xi - yi;
    }
}

fn main() {
    const split : usize = 100;
    const delta : f64 = 1.0 / (split as f64);
    let calc_pos = |xi: usize, yi: usize| {
        (xi - 1) + (yi - 1) * (split - 1)
    };
    let calc_x = |xi: usize| xi as f64 / (split as f64);
    let calc_y = |yi: usize| yi as f64 / (split as f64);
    let mut a : Vec<Vec<(usize,f64)>> =
        Vec::with_capacity(split * (split - 1));
    let mut b : Vec<f64> =
        Vec::with_capacity(split * (split - 1));
    for i in 0..(split * (split - 1)) {
        let mut vec : Vec<(usize,f64)> = Vec::with_capacity(5);
        let mut b_elem : f64 = 0.0f64;
        let xi = i % (split - 1) + 1;
        let yi = i / (split - 1) + 1;
        if yi < split {
            b_elem += delta.powi(2) * u_laplace(calc_x(xi), calc_y(yi));
            vec.push((calc_pos(xi, yi), 4.0));
            if xi > 1 {
                vec.push((calc_pos(xi - 1, yi), -1.0));
            } else {
                b_elem -= u(0.0, calc_y(yi));
            }
            if xi < split-1 {
                vec.push((calc_pos(xi + 1, yi), -1.0));
            } else {
                b_elem -= u(1.0, calc_y(yi));
            }
            if yi > 1 {
                vec.push((calc_pos(xi, yi - 1), -1.0));
            } else {
                b_elem -= u(calc_x(xi), 0.0);
            }
            vec.push((calc_pos(xi, yi + 1), -1.0));
        } else {
            b_elem -= delta * u_grad(calc_x(xi), 1.0).y;
            vec.push((calc_pos(xi, yi), 1.0));
            vec.push((calc_pos(xi, yi - 1), -1.0));
        }
        a.push(vec);
        b.push(b_elem);
    }
    let a : Vec<Vec<(usize,f64)>> = a;
    let b : Vec<f64> = b;
    let a_mul = |x : &Vec<f64>, y : &mut Vec<f64> | {
        for (i, yi) in y.iter_mut().enumerate() {
            let mut sum : f64 = 0.0f64;
            for &(j, aij) in a[i].iter() {
                sum += aij * x[j];
            }
            *yi = sum;
        }
    };
    let mut zeros : Vec<f64> = Vec::with_capacity(split * (split - 1));
    for _ in 0..split * (split - 1) { zeros.push(0.0f64); }
    let zeros = zeros;

    let bdot : f64 = vecdot(&b, &b);

    let mut x : Vec<f64> = zeros.clone();
    let mut y : Vec<f64> = zeros.clone();
    let mut r : Vec<f64> = zeros.clone();
    let mut ap : Vec<f64> = zeros.clone();
    a_mul(&x, &mut y);
    vecminus3(&y, &b, &mut r);
    let mut p : Vec<f64> = r.clone();
    loop {
        a_mul(&p, &mut ap);
        let pap : f64 = vecdot(&p, &ap);
        let rr : f64 = vecdot(&r, &r);
        let alpha : f64 = rr / pap;
        vecpluseq_alpha(&mut x, alpha, &p);
        vecpluseq_alpha(&mut r, -alpha, &ap);
        {
            let diff = vecdot(&r, &r) / bdot;
            writeln!(&mut std::io::stderr(), "diff = {}", diff);
            if diff < EPS {
                break;
            }
        }
        let rr_new : f64 = vecdot(&r, &r);
        let beta : f64 = rr_new / rr;
        for (pi, ri) in p.iter_mut().zip(r.iter()) {
            *pi = ri + beta * *pi;
        }
    }
    writeln!(&mut std::io::stderr(), "done!");
    let mut result_file = File::create("result.txt").unwrap();
    let mut expect_file = File::create("expect.txt").unwrap();
    let xvec = x;
    for xi in 0..split+1 {
        for yi in 0..split+1 {
            let x : f64 = calc_x(xi);
            let y : f64 = calc_y(yi);
            let z =
                if xi == 0 || yi == 0 || xi == split {
                    u(x, y)
                } else {
                    xvec[calc_pos(xi, yi)]
                };
            writeln!(&mut result_file, "{} {} {}", x, y, z);
            writeln!(&mut expect_file, "{} {} {}", x, y, u(x, y));
        }
        writeln!(&mut result_file, "");
        writeln!(&mut expect_file, "");
    }
}

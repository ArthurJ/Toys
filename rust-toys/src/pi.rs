//https://en.wikipedia.org/wiki/Bailey%E2%80%93Borwein%E2%80%93Plouffe_formula
//https://www.literateprograms.org/pi_with_the_bbp_formula__python_.html
use std::borrow::Borrow;

fn subp(over:usize, k8:usize, under:usize) -> f64{
    (over) as f64 / ((k8 + under) as f64)
}

pub fn bbp_parcel(k:usize) -> f64{
    let k8 = 8*k;
    1f64/16f64.powf(k as f64)
        * (subp(4, k8, 1)
            -subp(2, k8, 4)
            -subp(1, k8, 5)
            -subp(1, k8, 6))
}

pub fn pi_generator_fst() -> (usize, (usize, usize, usize, usize)) {
    pi_generator(1, 180, 60, 2)
}

pub fn pi_generator(q:usize, r:usize, t:usize, j:usize) -> (usize, (usize, usize, usize, usize)) {
    let (u, y) = (3 * (3 * j + 1) * (3 * j + 2), (q * (27 * j - 12) + 5 * r) / (5 * t));
    let (q_n, r_n, t_n, j_n) = (10 * q * j * (2 * j - 1), 10 * u * (q * (5 * j - 2) + r - y * t), t * u, j + 1);
    (y, (q_n, r_n, t_n, j_n))

// let (mut y, (mut q,mut  r,mut  t,mut  j)) = pi::pi_generator_fst();
// for i in 0..10{
//     (y, (q, r, t, j)) =  pi_generator(q, r, t, j);
//     println!("{:}",y);
// }

}
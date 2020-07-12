#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ret = 0.0;
    for i in 1..n + 1 {
        let mut tmp = 1.0 / n as f64;
        let mut now = i;
        while now < k {
            now = now * 2;
            tmp = tmp / 2.0;
        }
        ret += tmp;
    }
    println!("{:.12}", ret);
}

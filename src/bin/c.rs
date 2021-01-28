use proconio::input;
use itertools::Itertools;

fn main() {
    proconio::input! {
        n: i32,
        k: i32,
        t: [[i32; n]; n]
    }

    let mut routes = vec![];

    (1..n)
        .permutations((n-1) as usize)
        .for_each(|p| {
            let mut p_copy = p.clone();
            p_copy.insert(0, 0);
            p_copy.push(0);
            routes.push(p_copy);
        });

    let mut ans = 0;
    for route in routes{
        let mut time = 0;
        for i in 0..route.len() - 1 {
            time += t[route[i] as usize][route[i+1] as usize];
        }
        if time == k {
            ans += 1;
        }
    }

    println! ("{}",ans)
}
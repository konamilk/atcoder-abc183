use proconio::input;

fn main() {
    input! {
        x: i32
    }

    let ans = if x >= 0 {x} else {0};
    println!("{}", ans)
}

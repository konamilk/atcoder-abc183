use proconio::input;

fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64
    }

    let ans;

    if sx == gx{
        ans = sx as f64;
    }
    else {
        ans = -((sx - gx)* sy - (sy + gy) * sx) as f64 / (sy + gy) as f64;
    }

    println! ("{}",ans)
}

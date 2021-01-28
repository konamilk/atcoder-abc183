use proconio::input;

fn main() {
    input! {
        n:i64,
        w:i64,
        stp: [(i64, i64, i64); n]
    }

    let mut events = vec![];

    for elem in stp {
        events.push((elem.0, elem.2));
        events.push((elem.1, - elem.2));
    }

    events.sort();

    // println!("{:?}", events);

    let mut ans = "Yes";

    let mut usage = 0;
    let mut now = events[0].0;

    for (t, p) in events {
        if t != now {
            // println!("USAGE:{}", usage);
            if usage > w {
                ans = "No";
                break;
            }
        }
        usage += p;
        now = t;
    }

    println!("{}", ans);

}

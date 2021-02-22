use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let _n: i32 = iter.next().unwrap().parse().unwrap();
    let q: i32 = iter.next().unwrap().parse().unwrap();
    let mut queue: VecDeque<(String, i32)> = VecDeque::new();
    loop {
        let key = iter.next();
        if key == None {
            break;
        }
        queue.push_back((
            key.unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        ));
    }

    let mut time = 0;
    while 0 < queue.len() {
        let v = queue.pop_front().unwrap();
        if v.1 <= q {
            time += v.1;
            println!("{} {}", v.0, time);
            continue;
        }
        queue.push_back((v.0, v.1 - q));
        time += q;
    }
}

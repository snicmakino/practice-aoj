use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();

    let mut a: Vec<i32> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    // 処理
    let result = shell_sort(&mut a, n);
    println!("{}", result.0);
    println!(
        "{}",
        result
            .1
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!("{}", result.2);
    a.iter().for_each(|i| println!("{}", i));
}

fn insertion_sort(a: &mut Vec<i32>, n: i32, g: i32, cnt: &mut i32) {
    for i in g..n {
        let v = a[i as usize];
        let mut j = i - g;
        while j >= 0 && a[j as usize] > v {
            a[(j + g) as usize] = a[j as usize];
            j = j - g;
            *cnt += 1;
        }
        a[(j + g) as usize] = v;
    }
}

fn shell_sort(a: &mut Vec<i32>, n: i32) -> (usize, std::vec::Vec<i32>, i32) {
    let mut cnt = 0;
    let g = make_g(n);
    for i in 0..(g.len()) {
        insertion_sort(a, n, g[i], &mut cnt)
    }
    (g.len(), g, cnt)
}

fn make_g(n: i32) -> Vec<i32> {
    let mut g = vec![1];
    let mut x = 1;
    loop {
        x = (x * 3) + 1;
        if n < x {
            break;
        }
        g.push(x);
    }
    g.reverse();
    g
}

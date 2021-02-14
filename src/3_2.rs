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
    for i in 1..n {
        println_array(&a);
        let v = a[i as usize];
        let mut j = i - 1;
        while j >= 0 && a[j as usize] > v {
            a[(j + 1) as usize] = a[j as usize];
            j -= 1;
        }
        a[(j + 1) as usize] = v;
    }
    println_array(&a);
}

fn println_array(a: &Vec<i32>) {
    println!(
        "{}",
        a.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

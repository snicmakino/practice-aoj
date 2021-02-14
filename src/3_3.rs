use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut a: Vec<i32> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    // 処理
    let mut cnt = 0;
    for i in 1..n {
        for j in (i..n).rev() {
            if a[j] < a[j - 1] {
                a.swap(j, j - 1);
                cnt += 1;
            }
        }
    }
    println_array(&a);
    println!("{}", cnt);
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

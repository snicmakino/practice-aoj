use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let c: Vec<String> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    // 処理
    let bsorted = bubble_sort(n, c.clone());
    println_array(&bsorted);
    println!("{}", "Stable");

    let ssorted = selection_sort(n, c.clone());
    println_array(&ssorted);
    if bsorted == ssorted {
        println!("{}", "Stable");
    } else {
        println!("{}", "Not stable");
    }
}

fn bubble_sort(n: usize, mut c: Vec<String>) -> Vec<String> {
    for i in 1..n {
        for j in (i..n).rev() {
            if get_rank(&c[j]) < get_rank(&c[j - 1]) {
                c.swap(j, j - 1);
            }
        }
    }
    c
}

fn selection_sort(n: usize, mut c: Vec<String>) -> Vec<String> {
    for i in 0..n {
        let mut minj = i;
        for j in i..n {
            if get_rank(&c[j]) < get_rank(&c[minj]) {
                minj = j;
            }
        }
        if get_rank(&c[i]) != get_rank(&c[minj]) {
            c.swap(i, minj);
        }
    }
    c
}

fn get_rank(c: &String) -> i32 {
    c[1..2].parse::<i32>().unwrap()
}

fn println_array(c: &Vec<String>) {
    println!(
        "{}",
        c.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

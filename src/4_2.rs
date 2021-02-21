use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let iter = buf.split_whitespace();
    let mut stack: Vec<i32> = vec![];

    for v in iter {
        println!("{}", &v);
        match v {
            "+" => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y + x)
            }
            "-" => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y - x)
            }
            "*" => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y * x)
            }
            _ => stack.push(v.parse::<i32>().unwrap()),
        }
    }

    println!("{}", stack.pop().unwrap());
}

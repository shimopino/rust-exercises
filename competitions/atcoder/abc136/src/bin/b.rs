use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut count = 0;
    for i in 1..n+1 {
        if i.to_string().len() % 2 == 1 {
            count += 1;
        }
    }
    println!("{}", count);
}
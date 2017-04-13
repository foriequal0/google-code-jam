use std::io::{stdin, BufRead, Lines};
use std::str::FromStr;
use std::fmt::Debug;

fn solve<B>(lines: &mut Lines<B>) -> String
    where B: BufRead
{
    let prob: u64 = read(lines);

    let mut x = 0;
    let mut pow_of_10 = 1;

    let mut last_digit = 9;
    while prob / pow_of_10 > 0 {
        let digit = (prob / pow_of_10) % 10;

        if digit <= last_digit {
            x = digit * pow_of_10 + x;
            last_digit = digit;
        } else {
            let digit = digit - 1;
            x = digit * pow_of_10 + (pow_of_10 - 1);
            last_digit = digit;
        }
        
        pow_of_10 *= 10;
    }

    x.to_string()
}

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    let t:i32 = read(&mut lines);
    for case in 1..t+1 {
        let res = solve(&mut lines);
        println!("Case #{}: {}", case, res);
    }
}

fn read<B, F>(lines: &mut Lines<B>) -> F
    where B: BufRead, F: FromStr, <F as FromStr>::Err: Debug
{
    lines.next().unwrap().unwrap().parse().unwrap()
}

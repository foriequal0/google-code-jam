use std::io::{stdin, BufRead, Lines};
use std::str::FromStr;
use std::fmt::Debug;

struct Problem {
    n: u64,
    k: u64
}

#[derive(Debug)]
struct ParseProblemErr;

impl FromStr for Problem
{
    type Err = ParseProblemErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ws = s.split_whitespace();
        let n = ws.next().unwrap().parse().unwrap();
        let k = ws.next().unwrap().parse().unwrap();

        Result::Ok(Problem { n: n, k: k })
    }
}

fn solve<B>(lines: &mut Lines<B>) -> String
    where B: BufRead
{
    let p: Problem = read(lines);

    let n = p.n;
    let k = p.k;

    for depth in 0..64 {
        let pot = 1 << depth;
        let depth_sum = n - (pot-1);
        let residue = depth_sum % pot;

        if k >= pot && k < pot * 2 {
            if (k - pot) < residue {
                let x = depth_sum / pot + 1;
                return format!("{} {}", x/2, (x-1)/2);
            } else  {
                let x = depth_sum / pot;
                return format!("{} {}", x/2, (x-1)/2);
            }
        }
    }
    return format!("");
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

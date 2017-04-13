use std::io::{stdin, BufRead, Lines};
use std::str::FromStr;
use std::fmt::Debug;

struct Problem
{
    pancakes: Vec<bool>,
    k: usize,
}

#[derive(Debug)]
struct ParseProblemErr;

impl FromStr for Problem
{
    type Err = ParseProblemErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = s.split_whitespace();
        let pancakes = x.next().unwrap().chars()
            .map(|x| if x == '+' {true} else {false})
            .collect();
        let k = x.next().unwrap().parse().unwrap();
        Result::Ok(Problem { pancakes: pancakes, k: k })
    }
}

fn solve<B>(lines: &mut Lines<B>) -> String
    where B: BufRead
{
    let p: Problem = read(lines);
    let mut pancakes = p.pancakes;
    let mut count=0;
    for i in 0..pancakes.len() - p.k + 1 {
        if pancakes[i] == true {
            continue;
        }

        for j in i..(i + p.k) {
            pancakes[j] = !pancakes[j];
        }
        count += 1;
    }

    if pancakes.iter().all(|&x| x == true) {
        count.to_string()
    } else {
        "IMPOSSIBLE".to_string()
    }
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

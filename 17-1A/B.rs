use std::io::{stdin, Read};
use std::str::FromStr;

type Vec2<T> = Vec<Vec<T>>;

fn solve<R: Read>(words: &mut Words<R>) -> String {
    let n: usize = words.parse_next();
    let p: usize = words.parse_next();

    let r:Vec<i32> = (0..n).map(|_| words.parse_next()).collect();

    let mut q_min_max:Vec2<(i32, i32)> = vec![vec![]; n];
    for n in 0..n {
        let mut v = vec![];
        for _ in 0..p {
            let q_value = words.parse_next::<i32>();
            v.push(q_value);
        }
        v.sort();
        for p in 0..p {
            let min = div_ceil(10 * v[p], r[n] * 11);
            let max = (10 * v[p]) / (r[n] * 9);
            q_min_max[n].push((min, max));
        }
    }

    let mut mark: Vec2<bool> = vec![vec![false; p]; n];

    struct Ctx<'a> {
        n: usize,
        p: usize,
        qmm: &'a Vec2<(i32, i32)>,
        mark: &'a mut Vec2<bool>
    }
    
    fn walk(ctx: &mut Ctx, n: usize,  min: i32, max: i32) {
        if n >= ctx.n {
            return;
        }

        for p in 0..ctx.p {
            let (child_min, child_max) = ctx.qmm[n][p];
            let new_min = std::cmp::max(child_min, min);
            let new_max = std::cmp::min(child_max, max);
            
            if ctx.mark[n][p] == false && new_min <= new_max {
                ctx.mark[n][p] = true;
                walk(ctx, n+1, new_min, new_max);
                break;
            }
        }
    }
    
    {
        let mut ctx = Ctx { n: n, p:p, qmm: &q_min_max, mark: &mut mark };
        for _p in 0..p {
            walk(&mut ctx, 0,  0, std::i32::MAX);
        }
    }

    mark.iter().map(|line| line.iter().filter(|&&x| x).count()).min().unwrap().to_string()
}

fn div_ceil(x: i32, y: i32) -> i32 {
    let a = x / y;
    let b = x % y;

    if b == 0 {
        return a;
    } else {
        return a + 1;
    }
}

fn main() {
    let mut words = Words::new(stdin());
    let t: i32 = words.parse_next();
    for case in 1..t+1 {
        let res = solve(&mut words);
        println!("Case #{}: {}", case, res);
    }
}

struct Words<R> {
    inner: R
}

impl<R: Read> Words<R> {
    fn new(read: R) -> Words<R> {
        Words { inner: read }
    }

    fn parse_next<T>(&mut self) -> T where T: FromStr {
        self.next().and_then(|x| x.parse::<T>().ok()).unwrap()
    }
}

impl<R: Read> Iterator for Words<R> {
    type Item = String;
    
    fn next(&mut self) -> Option<String> {
        fn into_char(x: &[u8]) -> Option<char> {
            std::str::from_utf8(x).ok().and_then(|cs| cs.chars().nth(0))
        }
        fn into_nonempty_string(x: &[u8]) -> Option<String> {
            std::str::from_utf8(x).ok()
                .and_then(|x| if x.len() > 0 { Some(x) } else { None })
                .map(String::from)
        }
        let mut buf = vec![];
        let mut utf8_ends = 0;
        loop {
            let mut tmp = [0; 1];
            let read = self.inner.read(&mut tmp);
            if read.unwrap_or(0) > 0 {
                buf.extend_from_slice(&tmp);
            } else {
                return into_nonempty_string(&buf[0..utf8_ends]);
            }

            if let Some(last_char) = into_char(&buf[utf8_ends..]) {
                if last_char.is_whitespace() {
                    return into_nonempty_string(&buf[0..utf8_ends]);
                }
                utf8_ends = buf.len();
            }
        }
    }
}

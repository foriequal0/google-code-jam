use std::io::{stdin, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Rh {
    r: i64,
    h: i64
}

fn solve<R: Read>(words: &mut Words<R>) -> String {
    let n: usize = words.parse_next();
    let k: usize = words.parse_next();
    let rhs = {
        let mut x = vec![];
        for _ in 0..n {
            let r = words.parse_next();
            let h = words.parse_next();
            x.push(Rh { r: r, h: h })
        }
        x.sort_by(|x, y| {
            x.r.cmp(&y.r).reverse()
                .then(x.h.cmp(&y.h))
        });
        x
    };
    let sides: Vec<_> = rhs.iter()
        .map(|rh| 2 * rh.r * rh.h)
        .collect();
    let jump = {
        let mut vec = vec![n; n];
        let mut last = n;
        for i in (0..n-1).rev() {
            if rhs[i].r != rhs[i+1].r {
                vec[i] = i+1;
                last = i+1;
            } else {
                vec[i] = last;
            }
        }
        vec
    };

    let mut largest = 0;
    for base in (0..n-k+1).rev() {
        let mut area = rhs[base].r * rhs[base].r + sides[base];
        let mut x = Vec::from(&sides[base+1..n]);
        x.sort_by(|x, y| x.cmp(y).reverse());

        area += x.iter().take(k-1).sum();

        if largest < area {
            largest = area;
        }
    }
    
    const PI:f64 = std::f64::consts::PI;

    String::from(format!("{:.9}", largest as f64 * PI))
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

use std::io::{stdin, Read};
use std::str::FromStr;

fn solve<R: Read>(words: &mut Words<R>) -> String {
    let r: usize = words.parse_next().unwrap();
    let c: usize = words.parse_next().unwrap();

    let mut cake: Vec<Vec<char>> = vec![];
    
    for _ in 0..r {
        let row: String = words.parse_next().unwrap();
        cake.push(row.chars().collect());
    }

    for row in 0..r {
        let mut last = '?';
        for col in  (0..c).chain((0..c).rev()) {
            if last != '?' && cake[row][col] == '?' {
                cake[row][col] = last;
            } else if cake[row][col] != '?' {
                last = cake[row][col]
            }
        }
    }
    
    for col in 0..c {
        let mut last = '?';
        for row in  (0..r).chain((0..r).rev()) {
            if last != '?' && cake[row][col] == '?' {
                cake[row][col] = last;
            } else if cake[row][col] != '?' {
                last = cake[row][col]
            }
        }
    }

    let x = cake.iter().
        map(|chars| chars.iter().cloned().collect::<String>())
        .collect::<Vec<_>>();
    
    x.join("\n")
}

fn main() {
    let mut words = Words::new(stdin());
    let t: i32 = words.parse_next().unwrap();
    for case in 1..t+1 {
        let res = solve(&mut words);
        println!("Case #{}:\n{}", case, res);
    }
}

struct Words<R> {
    inner: R
}

impl<R: Read> Words<R> {
    fn new(read: R) -> Words<R> {
        Words { inner: read }
    }

    fn parse_next<T>(&mut self) -> Option<T> where T: FromStr {
        self.next().and_then(|x| x.parse::<T>().ok())
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

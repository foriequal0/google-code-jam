use std::io::{stdin, Read};
use std::str::FromStr;

fn solve<R: Read>(words: &mut Words<R>) -> String {
    let hd: u64 = words.parse_next();
    let ad: u64 = words.parse_next();
    let hk: u64 = words.parse_next();
    let ak: u64 = words.parse_next();
    let b: u64 = words.parse_next();
    let d: u64 = words.parse_next();

    let get_turns = |buffs| buffs + div_ceil(hk, ad + buffs * b);
    let buffs = (0..100001).min_by_key(|&x| get_turns(x)).unwrap();
    let atks = get_turns(buffs) - buffs;

    let possible_debuffs = {
        let mut v = vec![0];
        if d > 0 {
            let mut prev = 0;
            for debuffs in 0.. {
                if ak <= debuffs*d { break; }
                let curr = div_ceil(hd, ak-debuffs*d);
                if prev != curr {
                    v.push(curr);
                }
                prev = curr;
            }
        }
        v
    };
    
    let simulate = |target_debuffs| {
        let mut curr_hd = hd;
        let mut ad = ad;
        let mut hk = hk;
        let mut ak = ak;

        let mut debuffs = 0;
        let mut prev_health = false;
        let mut turn_count = 0;
        while debuffs < target_debuffs {
            
            turn_count += 1;
        }
    };
    
    return String::from(format!("{} {}", buffs, atks));
}

fn div_ceil(x: u64, y: u64) -> u64 {
    let a = x/y;
    match x % y {
        0 => return a,
        _ => return a+1
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

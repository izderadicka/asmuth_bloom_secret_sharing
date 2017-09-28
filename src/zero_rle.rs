// Run length encoding, but only for zeros
// zeros are encoded as 0xy - where xy is number of zeros as radix 32

use num::traits::{FromPrimitive};
use num::BigUint;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        DecodingError{}
        DecodingErrorInvalidZerosCount(err: ::std::num::ParseIntError) {
            cause(err)
            from()
        }
    }
}
fn push_zero(s: &mut String, count: &mut u16) {
    if *count == 0 {
        return;
    }
    s.push('0');
    let n = BigUint::from_u16(*count).unwrap().to_str_radix(32);
    assert!(n.len() > 0 && n.len() <= 2);
    if n.len() == 1 {
        s.push('0')
    }
    s.push_str(&n);
    *count = 0;
}
const MAX_ZEROS: u16 = 32 * 32 - 1;
pub fn encode(i: &str) -> String {
    let mut zero_count: u16 = 0;
    let mut res = String::new();

    for ch in i.chars() {
        if ch == '0' {
            if zero_count == MAX_ZEROS {
                push_zero(&mut res, &mut zero_count);
            }
            zero_count += 1;
        } else {
            push_zero(&mut res, &mut zero_count);
            res.push(ch)
        }
    }
    push_zero(&mut res, &mut zero_count);
    res
}

pub fn decode(i: &str) -> Result<String, Error> {
    let mut res = String::new();
    let mut zeros: Option<String> = None;
    for ch in i.chars() {
        match zeros.take() {
            Some(mut s) => if s.len() < 1 {
                s.push(ch);
                zeros = Some(s);
            } else {
                s.push(ch);
                let c = usize::from_str_radix(&s, 32)?;
                if c > MAX_ZEROS as usize {
                    return Err(Error::DecodingError);
                }
                for _ in 0..c {
                    res.push('0')
                }
            },
            None => if ch == '0' {
                zeros = Some(String::with_capacity(2));
            } else {
                res.push(ch)
            },
        }
    }

    if zeros.is_some() {
        return Err(Error::DecodingError);
    }

    Ok(res)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encode_decode() {
        fn t(s:&str) {
        let e = encode(s);
        println!("Encoded; {}", &e);
        let d = decode(&e).unwrap();
        assert_eq!(s, &d);
        }
        t("10000000000043");
        t("g00000000000d");
        t("1020030004000500006000000");

    }

}

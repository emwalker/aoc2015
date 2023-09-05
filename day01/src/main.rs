use color_eyre::{Report, Result};
use std::{
    io::{self, Read},
    str::FromStr,
};

struct Floors(String);

impl FromStr for Floors {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl Floors {
    fn floor(&self) -> isize {
        let mut v = 0;
        for c in self.0.chars() {
            match c {
                '(' => v += 1,
                ')' => v -= 1,
                _ => panic!("unknown character: {}", c),
            }
        }
        v
    }
}

fn main() -> Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    let res = s.trim().parse::<Floors>()?;

    println!("part 1: {}", res.floor());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floor(s: &str) -> isize {
        s.parse::<Floors>().unwrap().floor()
    }

    #[test]
    fn simple_cases() {
        assert_eq!(floor("(())"), 0);
        assert_eq!(floor("()()"), 0);
        assert_eq!(floor("((("), 3);
        assert_eq!(floor("(()(()("), 3);
        assert_eq!(floor("))((((("), 3);
        assert_eq!(floor("())"), -1);
        assert_eq!(floor(")))"), -3);
        assert_eq!(floor(")())())"), -3);
    }

    #[test]
    fn input() {
        let res = include_str!("../data/input.txt").parse::<Floors>().unwrap();
        assert_eq!(res.floor(), 138);
    }
}

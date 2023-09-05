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
        self.0.chars().fold(0, |acc, c| match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => panic!("unknown character: {}", c),
        })
    }

    fn position(&self, floor: isize) -> Option<usize> {
        let mut v = 0;

        for (i, c) in self.0.chars().enumerate() {
            match c {
                '(' => v += 1,
                ')' => v -= 1,
                _ => panic!("unknown character: {}", c),
            }

            if v == floor {
                return Some(i + 1);
            }
        }

        None
    }
}

struct Task {
    floors: Floors,
}

impl Task {
    fn part1(&self) -> isize {
        self.floors.floor()
    }

    fn part2(&self) -> usize {
        self.floors.position(-1).expect("floor not reached")
    }
}

fn parse(s: &str) -> Result<Task> {
    let floors = s.parse::<Floors>()?;
    Ok(Task { floors })
}

fn main() -> Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    let task = parse(&s)?;

    println!("part 1: {}", task.part1());
    println!("part 2: {}", task.part2());

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
    fn part2() {
        assert_eq!(parse(")").unwrap().part2(), 1);
        assert_eq!(parse("()())").unwrap().part2(), 5);
    }

    #[test]
    fn input() {
        let task = parse(include_str!("../data/input.txt")).unwrap();
        assert_eq!(task.part1(), 138);
        assert_eq!(task.part2(), 1771);
    }
}

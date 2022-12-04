use itertools::Itertools;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Assignment {
    first: RangeInclusive<i32>,
    second: RangeInclusive<i32>,
}

impl Assignment {
    fn from(s: &str) -> Assignment {
        let it = s.split(',').map(|s| {
            let b: Vec<i32> = s.split('-').map(
                |x| x.parse::<i32>().unwrap()).collect();
            RangeInclusive::new(b[0], b[1])
        });
        if let Some((first, second)) = it.collect_tuple() {
            Assignment { first, second }
        } else {
            panic!("unexpected size")
        }
    }

    fn contains(&self, reverse: bool, start: bool) -> bool {
        let a = if reverse { &self.second } else { &self.first };
        let b = if reverse { &self.first } else { &self.second };
        a.contains(if start { b.start() } else { b.end() })
    }

    fn overlap(&self) -> bool {
        let check = |r| self.contains(r, false) || self.contains(r, true);
        check(false) || check(true)
    }

    fn full_overlap(&self) -> bool {
        let check = |r| self.contains(r, false) && self.contains(r, true);
        check(false) || check(true)
    }
}

pub fn run(content: &str) {
    let input: Vec<Assignment> = content.lines().map(
        |s| Assignment::from(s)).collect();
    let overlaps: i32 = input.iter().map(|x| x.overlap() as i32).sum();
    let full_overlaps: i32 = input.iter().map(|x| x.full_overlap() as i32).sum();
    println!("{} {}", full_overlaps, overlaps);
}

#[cfg(test)]
mod tests {
    fn create(s: &str) -> super::Assignment {
        super::Assignment::from(s)
    }

    #[test]
    fn part1() {
        assert_eq!(create("2-4,6-8").full_overlap(), false);
        assert_eq!(create("2-3,4-5").full_overlap(), false);
        assert_eq!(create("5-7,7-9").full_overlap(), false);
        assert_eq!(create("2-8,3-7").full_overlap(), true);
        assert_eq!(create("6-6,4-6").full_overlap(), true);
        assert_eq!(create("2-6,4-8").full_overlap(), false);
    }

    #[test]
    fn part2() {
        assert_eq!(create("2-4,6-8").overlap(), false);
        assert_eq!(create("2-3,4-5").overlap(), false);
        assert_eq!(create("5-7,7-9").overlap(), true);
        assert_eq!(create("2-8,3-7").overlap(), true);
        assert_eq!(create("6-6,4-6").overlap(), true);
        assert_eq!(create("2-6,4-8").overlap(), true);
    }
}

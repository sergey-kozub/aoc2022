use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    content: String,
}

impl Rucksack {
    fn from(s: &str) -> Rucksack {
        Rucksack {
            content: s.to_string(),
        }
    }

    fn score(c: char) -> i32 {
        match c {
            'a'..='z' => (c as i32) - 96,
            'A'..='Z' => (c as i32) - 38,
            _ => panic!("incorrect item")
        }
    }

    fn priority(&self) -> i32 {
        let a = self.content.split_at(self.content.len() / 2);
        let s1 = HashSet::<char>::from_iter(a.0.chars());
        let s2 = HashSet::<char>::from_iter(a.1.chars());
        let c = s1.intersection(&s2).next().unwrap();
        Rucksack::score(*c)
    }

    fn badge(&self, a: &Rucksack, b: &Rucksack) -> i32 {
        let s1 = HashSet::<char>::from_iter(a.content.chars());
        let s2 = HashSet::<char>::from_iter(b.content.chars());
        let c = self.content.chars().filter(
            |v| s1.contains(v) && s2.contains(v)).next().unwrap();
        Rucksack::score(c)
    }
}

pub fn run(content: &str) {
    let input: Vec<Rucksack> = content.lines().map(Rucksack::from).collect();
    let sum_priorities: i32 = input.iter().map(|r| r.priority()).sum();
    let sum_badges: i32 = (0..input.len()).step_by(3).map(
        |i| input[i].badge(&input[i + 1], &input[i + 2])).sum();
    println!("{} {}", sum_priorities, sum_badges)
}

#[cfg(test)]
mod tests {
    fn create(s: &str) -> super::Rucksack {
        super::Rucksack::from(s)
    }

    #[test]
    fn part1() {
        assert_eq!(create("vJrwpWtwJgWrhcsFMMfFFhFp").priority(), 16);
        assert_eq!(create("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").priority(), 38);
        assert_eq!(create("PmmdzqPrVvPwwTWBwg").priority(), 42);
        assert_eq!(create("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").priority(), 22);
        assert_eq!(create("ttgJtRGJQctTZtZT").priority(), 20);
        assert_eq!(create("CrZsJsPPZsGzwwsLwLmpwMDw").priority(), 19);
    }

    #[test]
    fn part2() {
        assert_eq!(
            create("vJrwpWtwJgWrhcsFMMfFFhFp").badge(
            &create("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            &create("PmmdzqPrVvPwwTWBwg")), 18);
        assert_eq!(
            create("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").badge(
            &create("ttgJtRGJQctTZtZT"),
            &create("CrZsJsPPZsGzwwsLwLmpwMDw")), 52);
    }
}

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));

fn main() {
    let masses = split_input(INPUT);

    let answer_one = part_1(&masses);

    println!("answer to part one is: {answer_one}");

    let answer_two = part_2(&masses);

    println!("answer to part two is: {answer_two}")
}

fn part_1(masses: &[i32]) -> i32 {
    masses.iter().copied().map(get_fuel).sum()
}

fn part_2(masses: &[i32]) -> i32 {
    masses.iter().copied().map(get_fuel_for_fuel).sum()
}

fn get_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn get_fuel_for_fuel(mass: i32) -> i32 {
    std::iter::successors(Some(get_fuel(mass)), |&f| Some(get_fuel(f)))
        .take_while(|&f| f > 0)
        .sum()
}

fn split_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().expect("value should be an integer"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fuel() {
        for (mass, expected) in [(12, 2), (14, 2), (1969, 654), (100756, 33583)] {
            assert_eq!(get_fuel(mass), expected, "mass {mass}")
        }
    }

    #[test]
    fn test_get_fuel_for_fuel() {
        for (mass, expected) in [(14, 2), (1969, 966), (100756, 50346)] {
            assert_eq!(get_fuel_for_fuel(mass), expected, "mass {mass}")
        }
    }

    #[test]
    fn test_split_input() {
        let input = "1\n2\n3";
        assert_eq!(split_input(input), vec![1, 2, 3]);
    }
}

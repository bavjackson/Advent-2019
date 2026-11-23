fn main() {
    let input = utils::read_input("./input.txt");

    let masses = split_input(&input);

    let answer_one = part_1(&masses);

    println!("answer to part one is: {answer_one}");

    let answer_two = part_2(&masses);

    println!("answer to part two is: {answer_two}")
}

fn part_1(masses: &[i32]) -> i32 {
    masses.iter().map(get_fuel).sum()
}

fn part_2(masses: &[i32]) -> i32 {
    masses.iter().map(get_fuel_for_fuel).sum()
}

fn get_fuel(mass: &i32) -> i32 {
    (mass / 3) - 2
}

fn get_fuel_for_fuel(mass: &i32) -> i32 {
    let fuel = get_fuel(mass);
    if fuel < 0 {
        0
    } else {
        fuel + get_fuel_for_fuel(&fuel)
    }
}

fn split_input(input: &str) -> Vec<i32> {
    input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().expect("value should be an integer"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fuel() {
        let inputs: [i32; 4] = [12, 14, 1969, 100756];
        let outputs: [i32; 4] = [2, 2, 654, 33583];

        for (i, v) in inputs.iter().enumerate() {
            assert_eq!(get_fuel(v), outputs[i])
        }
    }

    #[test]
    fn test_get_fuel_for_fuel() {
        let inputs: [i32; 3] = [14, 1969, 100756];
        let outputs: [i32; 3] = [2, 966, 50346];

        for (i, v) in inputs.iter().enumerate() {
            assert_eq!(get_fuel_for_fuel(v), outputs[i])
        }
    }

    #[test]
    fn test_split_input() {
        let input = "1
2
3";
        assert_eq!(split_input(input), vec![1, 2, 3]);
    }
}

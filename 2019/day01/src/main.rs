fn calc_fuel(mass: i32) -> i32 {
    let result = ((mass as f32 / 3_f32).floor()) as i32 - 2;
    if result <= 0 {
        0
    } else {
        result + calc_fuel(result)
    }
}

pub fn part1(input: &str) -> i32 {
    return input.lines().fold(0, |acc, x| {
        let current_mass = x.parse::<i32>().unwrap();
        let result = ((current_mass as f32 / 3_f32).floor()) as i32 - 2;
        acc + result
    });
}

pub fn part2(input: &str) -> i32 {
    return input.lines().fold(0, |acc, x| {
        let current_mass = x.parse::<i32>().unwrap();
        let result = calc_fuel(current_mass);
        acc + result
    });
}

fn main() {
    println!("part1 {}", part1(include_str!("../input.txt")));
    println!("part2 {}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn part1_works() {
        let foo = part1(include_str!("../inputTestData.txt"));
        assert_eq!(foo, 34241);
    }

    #[test]
    fn part2_works() {
        let foo = part2(include_str!("../inputTestData2.txt"));
        assert_eq!(foo, 51314);
    }
}

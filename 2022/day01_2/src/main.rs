fn main() {
    let input_data = include_str!("../input.txt");
    let calories_per_elve = input_data.split("\n\n");

    let mut max = 0;

    let mut callories_carried = Vec::new();

    for s in calories_per_elve {
        let mut current_max = 0;
        for line in s.lines() {
            current_max = current_max + line.parse::<i32>().unwrap();
        }
        callories_carried.push(current_max);

        if current_max > max {
            max = current_max;
        }
    }
    println!("Max: {}", max);

    callories_carried.sort();

    let e1 = callories_carried.pop().unwrap();
    let e2 = callories_carried.pop().unwrap();
    let e3 = callories_carried.pop().unwrap();

    let sum_of_first_three = e1 + e2 + e3;

    println!("sum {:?},  ", sum_of_first_three);
}

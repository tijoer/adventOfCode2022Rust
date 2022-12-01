fn main() {
    let input_data = include_str!("../input.txt");
    let calories_per_elve = input_data.split("\n\n");

    let mut max = 0;
    for s in calories_per_elve {
        let mut current_max = 0;
        for line in s.lines() {
            current_max = current_max + line.parse::<i32>().unwrap();
        }

        if current_max > max {
            max = current_max;
        }

        println!("{}", max);
    }
}

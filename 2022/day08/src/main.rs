#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]

fn part1(input: &str) -> i32 {
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as usize - '0' as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let mut num_visible = 0;

    for x in 1..trees.len() - 1 {
        println!("");
        for y in 1..trees.get(0).unwrap().len() - 1 {
            let value = trees[x][y];

            let row = &trees[x];
            let col = trees.iter().map(|row| row[y]).collect::<Vec<_>>();

            let left_value = row[y - 1];
            let right_value = row[y + 1];
            let top_value = col[x - 1 ];
            let bottom_value = col[x + 1];

            if value > left_value || value > right_value || value > top_value || value > bottom_value {
                num_visible += 1;
            }

            // print!("  {} {} {}   ", top_value, value, bottom_value);
        }
    }
    println!("");
    // TODO replace 16 with calculation
    return num_visible + 16;
}

fn main() {
    println!("day07 part1: {}", part1(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_works() {
        // part1(include_str!("../inputTestData.txt"));
        assert_eq!(part1(include_str!("../inputTestData.txt")), 21);
    }
}

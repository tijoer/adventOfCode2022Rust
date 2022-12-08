fn solve(input: &str) -> (i32, i32) {
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as usize - '0' as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let mut num_visible = 0;
    let mut scenic_score = 0;
    let mut highest_metric = 0;

    for x in 1..trees.len() - 1 {
        // println!("");
        for y in 1..trees.get(0).unwrap().len() - 1 {
            let value = &trees[x][y];

            let row = &trees[x];
            let col = trees.iter().map(|row| row[y]).collect::<Vec<_>>();

            let left_highest_value = &row[0..y].iter().max().unwrap();
            let right_highest_value = &row[y + 1..row.len()].iter().max().unwrap();
            let top_highest_value = &col[0..x].iter().max().unwrap();
            let bottom_highest_value = &col[x + 1..col.len()].iter().max().unwrap();

            if value > left_highest_value
                || value > right_highest_value
                || value > top_highest_value
                || value > bottom_highest_value
            {
                num_visible += 1;
            }

            let left = &row[0..y];
            let left_num = left
                .iter()
                .rev()
                .position(|el| el >= value)
                .unwrap_or(left.len() - 1)
                + 1;

            let right = &row[y + 1..row.len()];
            let right_num = right
                .iter()
                .position(|el| el >= value)
                .unwrap_or(right.len() - 1)
                + 1;

            let up = &col[0..x];
            let up_num = up
                .iter()
                .rev()
                .position(|el| el >= value)
                .unwrap_or(up.len() - 1)
                + 1;

            let down = &col[x + 1..col.len()];
            let down_num = down
                .iter()
                .position(|el| el >= value)
                .unwrap_or(down.len() - 1)
                + 1;

            let metric = left_num * right_num * up_num * down_num;
            if metric > highest_metric {
                highest_metric = metric;
            }
            //  print!("  {} {} {}   ", left_highest_value, value, right_highest_value);
        }
    }
    // println!("");
    // TODO replace 16 with calculation
    (
        num_visible + (trees.len() * 2 + trees[0].len() * 2 - 4) as i32,
        highest_metric as i32,
    )
}

fn main() {
    println!("day07 part1: {}", solve(include_str!("../input.txt")).0);
    println!("day07 part1: {}", solve(include_str!("../input.txt")).1);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part2_works() {
        // part1(include_str!("../inputTestData.txt"));
        assert_eq!(solve(include_str!("../inputTestData.txt")).1, 8);
    }

    #[test]
    fn part1_works() {
        // part1(include_str!("../inputTestData.txt"));
        assert_eq!(solve(include_str!("../inputTestData.txt")).0, 21);
    }
}

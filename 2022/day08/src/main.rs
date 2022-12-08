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

    for x in 1..trees.len() - 1 {
        // println!("");
        for y in 1..trees.get(0).unwrap().len() - 1 {
            let value = &trees[x][y];

            let row = &trees[x];
            let col = trees.iter().map(|row| row[y]).collect::<Vec<_>>();

            if value > (&row[0..y].iter().max().unwrap())
                || value > (&row[y + 1..row.len()].iter().max().unwrap())
                || value > (&col[0..x].iter().max().unwrap())
                || value > (&col[x + 1..col.len()].iter().max().unwrap())
            {
                num_visible += 1;
            }

            let left_num = calc_left(&row[0..y].to_vec(), y, value);
            let right_num = calc_right(&row[y + 1..row.len()].to_vec(), y, value);
            let up_num = calc_up(&col[0..x].to_vec(), value);
            let down_num = calc_down(&col[x + 1..col.len()].to_vec(), x, value);

            let current_score = left_num * right_num * up_num * down_num;
            if (left_num * right_num * up_num * down_num) > scenic_score {
                scenic_score = current_score;
            }
            //  print!("  {} {} {}   ", left_highest_value, value, right_highest_value);
        }
    }
    // println!("");
    (
        num_visible + (trees.len() * 2 + trees[0].len() * 2 - 4) as i32,
        scenic_score as i32,
    )
}

fn calc_left(row: &Vec<usize>, y: usize, value: &usize) -> usize {
    let left_num =row
        .iter()
        .rev()
        .position(|&p| &p >= value)
        .unwrap_or(row.len() - 1)
        + 1;
    left_num
}

fn calc_right(row: &Vec<usize>, y: usize, value: &usize) -> usize {
    let right_num = row
        .iter()
        .position(|&p| &p >= value)
        .unwrap_or(row.len() - 1)
        + 1;
    right_num
}

fn calc_up(col: &Vec<usize>, value: &usize) -> usize {
    let up_num = col
        .iter()
        .rev()
        .position(|&p| &p >= value)
        .unwrap_or(col.len() - 1)
        + 1;
    up_num
}

fn calc_down(col: &Vec<usize>, x: usize, value: &usize) -> usize {
    let down_num = col
        .iter()
        .position(|&p| &p >= value)
        .unwrap_or(col.len() - 1)
        + 1;
    down_num
}

fn main() {
    println!("day08 part1: {}", solve(include_str!("../input.txt")).0);
    println!("day08 part2: {}", solve(include_str!("../input.txt")).1);
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

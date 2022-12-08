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
        for y in 1..trees.get(0).unwrap().len() - 1 {
            let value = &trees[x][y];

            let row = &trees[x];
            let col = trees.iter().map(|row| row[y]).collect::<Vec<_>>();

            if value > (row[0..y].iter().max().unwrap())
                || value > (row[y + 1..row.len()].iter().max().unwrap())
                || value > (col[0..x].iter().max().unwrap())
                || value > (col[x + 1..col.len()].iter().max().unwrap())
            {
                num_visible += 1;
            }
        }
    }
    num_visible + (trees.len() * 2 + trees[0].len() * 2 - 4) as i32
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

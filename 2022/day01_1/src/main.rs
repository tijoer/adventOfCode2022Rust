fn part1() {
    let input_data = include_str!("../input.txt")
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|item| return item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    println!("{}", input_data);
}

fn part2() {
    let input_data = include_str!("../input.txt");
    let mut _result = input_data
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|item| return item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

        // let foo = _result.to
        // _result.sort_by(|a, b| b.cmp(a));

        // .iter().take(3).sum();

    // _result.sort_by(|a, b| b.cmp(a));
    // let sum: u32 = _result.iter().take(3).sum();
    dbg!(_result);
    // dbg!(sum);

    // let foo: u32 = _result.iter().rev().take(3).sum();

    // println!("{:?}", foo);
}

fn main() {
    part1();
    part2();
}

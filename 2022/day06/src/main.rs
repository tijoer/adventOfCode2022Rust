use std::collections::HashSet;

fn find_start_of_packet_marker(input: &str, msg_size: usize) -> i32 {
    let mut result = 0;

    input.chars().collect::<Vec<char>>().windows(msg_size).fold(0, |acc, nxt| {
        let set: HashSet<_> = nxt.into_iter().collect::<HashSet<_>>();
        if set.len() == msg_size && result == 0 {
            result = acc + msg_size;
        }
        return acc + 1;
    });

    return result as i32;
}

fn main() {
    println!(
        "day05 part1: {}",
        find_start_of_packet_marker(include_str!("../input.txt"), 14)
    );
}

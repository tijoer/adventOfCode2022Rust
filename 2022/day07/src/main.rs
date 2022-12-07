#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]

#[derive(Debug, Clone)]
struct Folder {
    name: String,
    size: i32,
    subfolders: Vec<Folder>,
}

impl Folder {
    pub fn build_tree<'a>(&'a mut self, lines: &mut Vec<&str>) -> &'a Folder {
        while lines.len() != 0 {
            let line = lines
                .drain(0..1)
                .into_iter()
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .trim();

            let commands = line.split(' ').collect::<Vec<&str>>();
            let command = commands.get(1).unwrap().trim();
            let first_word = commands.get(0).unwrap().trim();

            if command.eq("cd") {
                let subcommand = commands.get(2).unwrap().trim();

                if subcommand.eq("..") {
                    return self;
                } else {
                    //cd into subfolder
                    let mut new_folder = Folder {
                        name: subcommand.to_string(),
                        size: 0,
                        subfolders: Vec::new(),
                    };
                    new_folder.build_tree(lines);
                    self.size += new_folder.size;

                    self.subfolders.push(new_folder);
                }
            }

            if first_word.eq("$") || first_word.eq("dir") {
                continue;
            } else {
                self.size += first_word.parse::<i32>().unwrap();
            }
        }

        return self;
    }

    pub fn calculate_to_at_most_100000(&self) -> i32 {
        let mut result = 0;
        if self.size <= 100000 {
            result += self.size;
        }

        result += self
            .subfolders
            .iter()
            .map(|f| return f.calculate_to_at_most_100000())
            .sum::<i32>();

        return result;
    }

    fn get_all_sizes() {}

    pub fn find_smallest_folder_size(&self) -> Vec<i32> {
        let mut sizes: Vec<i32> = Vec::new();
        sizes.push(self.size);

        self.subfolders.iter().for_each(|f| {
            let sizes_from_subfolders: Vec<i32> = f.find_smallest_folder_size();
            sizes_from_subfolders.iter().for_each(|size| {
               sizes.push(*size);
            });
            // sizes.append(sizes_from_subfolders);
        });

        return sizes;
    }
}

fn part1(input: &str) -> i32 {
    let mut lines = input.lines().collect::<Vec<&str>>();

    let mut root = Folder {
        name: "/".to_string(),
        size: 0,
        subfolders: Vec::new(),
    };

    root.build_tree(&mut lines);

    let result = root.calculate_to_at_most_100000();
    return result;
}

fn part2(input: &str) -> i32 {
    let mut lines = input.lines().collect::<Vec<&str>>();

    let mut root = Folder {
        name: "/".to_string(),
        size: 0,
        subfolders: Vec::new(),
    };

    root.build_tree(&mut lines);
    let min_space_to_free = 70000000 - root.size;
    dbg!(min_space_to_free);

    let mut result = root.find_smallest_folder_size();
    result.sort();

    dbg!(result); //2292394,

    // let mut foo = 0;
    // for i in 0..result.len() {
    //     if *result.get(i).unwrap() >= min_space_to_free {
    //         return *result.get(i).unwrap();
    //     }
    // };
    
    return 0;
}

fn main() {
    println!("day07 part1: {}", part1(include_str!("../input.txt")));
    println!("day07 part2: {}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1(include_str!("../inputTestData.txt")), 95437);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(include_str!("../inputTestData.txt")), 24933642);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]

#[derive(Debug)]
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

            if command.eq("cd") {
                let subcommand =  commands.get(2).unwrap().trim();

                if subcommand.eq("..") {
                    return self
                } else {
                    //cd into subfolder
                    let mut new_folder = Folder {
                        name: subcommand.to_string(),
                        size: 0,
                        subfolders: Vec::new()
                    };
                    new_folder.build_tree(lines);

                    self.subfolders.push(new_folder);
                }
                println!("cd {}", subcommand);
            }
        }

        return self;
    }
}

fn solve(input: &str) -> i32 {
    let mut lines = input.lines().collect::<Vec<&str>>();
    
    let mut root = Folder {
        name: "/".to_string(),
        size: 0,
        subfolders: Vec::new(),
    };

    root.build_tree(&mut lines);
    dbg!(root);

    return 0;
}

fn main() {
    println!("day07 part1: {}", solve(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn part1_works() {
        assert_eq!(solve(include_str!("../inputTestData.txt")), 95437);
    }
}

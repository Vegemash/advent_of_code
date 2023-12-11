use indextree::{Arena, Children, Node, NodeId};

#[derive(Debug)]
struct FileNode {
    size: usize,
    name: String,
}

pub fn process_part_1(input: &str) -> usize {
    let (root, tree) = parse_tree(input);
    root.descendants(&tree)
        .filter_map(|node_id| tree.get(node_id))
        .map(|node| {
            let data = node.get();
            if data.size == 0 {
                // println!("processing node {}", data.name);
                let node_size = get_node_size(node, &tree);
                if node_size > 100_000 {
                    return 0;
                }
                // println!("found size: {node_size} {}", data.name);
                node_size
            } else {
                0
            }
        })
        .sum()
}

fn get_node_size(node: &Node<FileNode>, filesystem: &Arena<FileNode>) -> usize {
    let mut node_size = 0;
    let data = node.get();
    node_size += data.size;
    if let Some(node_id) = node.first_child() {
        node_size += get_node_size(filesystem.get(node_id).unwrap(), &filesystem);
        let mut sibling = filesystem.get(node_id).unwrap();
        while let Some(sibling_id) = sibling.next_sibling() {
            sibling = filesystem.get(sibling_id).unwrap();
            node_size += get_node_size(sibling, &filesystem);
        }
    }

    node_size
}

pub fn process_part_2(input: &str) -> usize {
    let (root, tree) = parse_tree(input);
    // println!("{}", get_node_size(tree.get(root).unwrap(), &tree));
    let total_used = get_node_size(tree.get(root).unwrap(), &tree);
    let required = 30_000_000 - (70_000_000 - total_used);
    let mut chosen = std::usize::MAX;
    for node in root
        .descendants(&tree)
        .filter_map(|node_id| tree.get(node_id))
    {
        if node.get().size == 0 {
            // println!("processing node {}", data.name);
            let node_size = get_node_size(node, &tree);
            if node_size >= required && node_size < chosen {
                chosen = node_size;
                println!(
                    "required: {}. {} - {} = {}",
                    required,
                    total_used,
                    chosen,
                    total_used - chosen
                );
            }
        }
    }
    chosen
}

fn parse_tree(input: &str) -> (NodeId, Arena<FileNode>) {
    let mut filesystem = Arena::new();
    let root = filesystem.new_node(FileNode {
        size: 0,
        name: "/".to_string(),
    });
    let mut current_dir = root.clone();

    for line in input.lines() {
        // handle commands
        if line.starts_with("$") {
            if line.starts_with("$ ls") {
                continue;
            }
            if line == "$ cd .." && current_dir != root {
                current_dir = filesystem.get(current_dir).unwrap().parent().unwrap();
                continue;
            }
            if line == "$ cd /" {
                current_dir = root;
                continue;
            }
            current_dir = filesystem
                .get_node_id(
                    current_dir
                        .children(&filesystem)
                        .filter_map(|id| filesystem.get(id))
                        .find_map(|node| {
                            if node.get().name
                                == line
                                    .split(" ")
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>()[2]
                            {
                                Some(node)
                            } else {
                                None
                            }
                        })
                        .unwrap(),
                )
                .unwrap();
        } else {
            let (size_str, name) = line.split_once(" ").unwrap();

            let mut size = 0;
            if !line.starts_with("dir") {
                size = size_str.parse::<usize>().unwrap();
            }
            let child = filesystem.new_node(FileNode {
                size,
                name: name.to_string(),
            });
            if size > 0 {
                // println!("{}: {}", size, name);
            }

            current_dir.append(child, &mut filesystem);
        }
    }

    (root, filesystem)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part_1_works() {
        let result = process_part_1(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        );
        assert_eq!(result, 95437);
    }
    #[test]
    fn process_part_2_works() {
        let result = process_part_2(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        );
        assert_eq!(result, 24933642);
    }
}

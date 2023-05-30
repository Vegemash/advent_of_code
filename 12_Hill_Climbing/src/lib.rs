use std::cmp::Ordering;
#[derive(Debug, PartialEq, Copy, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum PathDistance {
    Unvisited,
    Visited(usize),
}

trait PathDistanceValue {
    fn value(&self) -> Option<usize>;
}
impl PathDistanceValue for PathDistance {
    fn value(&self) -> Option<usize> {
        match self {
            PathDistance::Visited(x) => Some(*x),
            PathDistance::Unvisited => None,
        }
    }
}

impl Ord for PathDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        match self {
            PathDistance::Unvisited => Ordering::Greater,
            PathDistance::Visited(x) => match other {
                PathDistance::Unvisited => Ordering::Less,
                PathDistance::Visited(y) => x.cmp(y),
            },
        }
    }
}

impl PartialOrd for PathDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct PathNode {
    distance: PathDistance,
    height: usize,
}

#[derive(Debug)]
struct HillMap {
    width: usize,
    height: usize,
    tiles: Vec<Vec<PathNode>>,
}

trait Dyk {
    fn new(input: &str) -> Self;

    fn get_neigbors(&self, coords: Coords) -> Vec<Coords>;
    fn get_rev_neigbors(&self, coords: Coords) -> Vec<Coords>;

    fn val_at(&self, coords: Coords) -> usize;
    fn dist_at(&self, coords: Coords) -> PathDistance;

    fn is_processed(&self, coords: Coords) -> bool;
    fn set_dist_at(&mut self, coords: Coords, dist: PathDistance) -> ();
    fn get_grid_neighbors(&self, coords: Coords) -> Vec<Coords>;
    fn print_hill(&self) -> ();
}

impl Dyk for HillMap {
    fn print_hill(&self) -> () {
        print!("\n======================\n");
        for y in 0..self.height {
            for x in 0..self.width {
                match self.dist_at(Coords { x, y }) {
                    PathDistance::Visited(d) => print!("{}, ", d),
                    PathDistance::Unvisited => print!(". "),
                }
            }
            print!("\n");
        }
    }
    fn set_dist_at(&mut self, coords: Coords, dist: PathDistance) -> () {
        self.tiles[coords.y][coords.x].distance = dist;
    }
    fn is_processed(&self, coords: Coords) -> bool {
        match self.tiles[coords.y][coords.x].distance {
            PathDistance::Unvisited => false,
            PathDistance::Visited(_) => true,
        }
    }

    fn new(input: &str) -> Self {
        let mut new = HillMap {
            tiles: vec![],
            width: 0,
            height: 0,
        };

        for lyne in input.split_terminator("\n") {
            let row = lyne
                .chars()
                .map(|x| PathNode {
                    distance: PathDistance::Unvisited,
                    height: match x {
                        'S' => 'a',
                        'E' => 'z',
                        _ => x,
                    } as usize,
                })
                .collect::<Vec<PathNode>>();

            new.width = row.len();
            new.height += 1;
            new.tiles.push(row);
        }
        let start_tile = find_char(input, 'S').unwrap();
        new.tiles[start_tile.y][start_tile.x].distance = PathDistance::Visited(0);
        new
    }

    fn val_at(&self, coords: Coords) -> usize {
        self.tiles[coords.y][coords.x].height
    }
    fn dist_at(&self, coords: Coords) -> PathDistance {
        self.tiles[coords.y][coords.x].distance
    }

    fn get_grid_neighbors(&self, coords: Coords) -> Vec<Coords> {
        let mut neigbors = vec![];
        if coords.x > 0 {
            neigbors.push(Coords {
                x: coords.x - 1,
                y: coords.y,
            });
        }
        if coords.x < self.width - 1 {
            neigbors.push(Coords {
                x: coords.x + 1,
                y: coords.y,
            });
        }

        if coords.y > 0 {
            neigbors.push(Coords {
                x: coords.x,
                y: coords.y - 1,
            });
        }
        if coords.y < self.height - 1 {
            neigbors.push(Coords {
                x: coords.x,
                y: coords.y + 1,
            });
        }
        neigbors
    }
    fn get_neigbors(&self, coords: Coords) -> Vec<Coords> {
        self.get_grid_neighbors(coords)
            .iter()
            .filter(|&v| self.val_at(*v) <= self.val_at(coords) + 1)
            .map(|&x| x)
            .collect::<Vec<Coords>>()
    }
    fn get_rev_neigbors(&self, coords: Coords) -> Vec<Coords> {
        self.get_grid_neighbors(coords)
            .iter()
            .filter(|&v| self.val_at(*v) >= self.val_at(coords).saturating_sub(1))
            .map(|&x| x)
            .collect::<Vec<Coords>>()
    }
}

pub fn process_part_1(input: &str) -> usize {
    let mut hill = HillMap::new(input);
    let start_tile = find_char(input, 'S').unwrap();
    let mut unprocessed_nodes: Vec<Coords> = vec![start_tile.clone()];
    hill.set_dist_at(start_tile, PathDistance::Visited(0));
    let endpoint = find_char(input, 'E').unwrap();

    loop {
        // Get Shortest unprocessed node
        unprocessed_nodes.sort_by_key(|&x| hill.dist_at(x));
        unprocessed_nodes.reverse();

        let processing_node = unprocessed_nodes.pop().unwrap();
        // Get neigbors
        let new_nodes = hill
            .get_neigbors(processing_node)
            .iter()
            // Filter out processed nodes
            .filter(|x| !hill.is_processed(*x.clone()))
            .map(|&x| x)
            .collect::<Vec<Coords>>();

        //process nodes
        for new_coord in new_nodes.iter() {
            hill.set_dist_at(
                *new_coord,
                PathDistance::Visited(hill.dist_at(processing_node).value().unwrap() + 1),
            );
            // Stop if we hit the endpoint
            if *new_coord == endpoint {
                return hill.dist_at(endpoint).value().unwrap();
            }

            unprocessed_nodes.push(*new_coord);
        }
    }
}

pub fn process_part_2(input: &str) -> usize {
    let mut hill = HillMap::new(input);
    let start_tile = find_char(input, 'E').unwrap();
    let mut unprocessed_nodes: Vec<Coords> = vec![start_tile];
    hill.set_dist_at(start_tile, PathDistance::Visited(0));

    loop {
        // Get Shortest unprocessed node
        unprocessed_nodes.sort_by_key(|&x| hill.dist_at(x));
        unprocessed_nodes.reverse();

        let processing_node = unprocessed_nodes.pop().unwrap();
        // Get neigbors
        let new_nodes = hill
            .get_rev_neigbors(processing_node)
            .iter()
            // Filter out processed nodes
            .filter(|x| !hill.is_processed(*x.clone()))
            .map(|&x| x)
            .collect::<Vec<Coords>>();

        //process nodes
        for new_coord in new_nodes.iter() {
            hill.set_dist_at(
                *new_coord,
                PathDistance::Visited(hill.dist_at(processing_node).value().unwrap() + 1),
            );
            // Stop if we hit the endpoint
            if hill.val_at(*new_coord) == 'a' as usize {
                return hill.dist_at(*new_coord).value().unwrap();
            }

            unprocessed_nodes.push(*new_coord);
        }
    }
}

fn find_char(input: &str, target: char) -> Option<Coords> {
    for (y, lyne) in input
        .split("\n")
        .enumerate()
        .map(|(x, line)| (x, line.to_string()))
    {
        for (x, ch) in lyne.chars().enumerate() {
            if ch == target {
                return Some(Coords { x, y });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(Coords { x: 0, y: 0 }, vec![Coords { x: 1, y: 0 }, Coords { x: 0, y: 1 }])]
    #[case(Coords { x: 2, y: 0 }, vec![Coords { x: 1, y: 0 }, Coords { x: 2, y: 1 }])]
    #[
        case(
            Coords { x: 3, y: 1 },
            vec![
                Coords { x: 2, y: 1 },
                Coords { x: 3, y: 0 },
                Coords { x: 3, y: 2 },
            ],
        )
    ]
    fn test_get_neigbors(#[case] coords: Coords, #[case] neigbors: Vec<Coords>) {
        let hmap = HillMap::new(include_str!("../data/test_input.txt"));
        assert_eq!(hmap.get_neigbors(coords), neigbors);
    }
    #[test]
    #[allow(non_snake_case)]
    fn test_find_S() {
        let result = find_char(include_str!("../data/test_input.txt"), 'S');
        assert_eq!(result, Some(Coords { x: 0, y: 0 }));
    }
    #[test]
    #[allow(non_snake_case)]
    fn test_find_E() {
        let result = find_char(include_str!("../data/test_input.txt"), 'E');
        assert_eq!(result, Some(Coords { x: 5, y: 2 }));
    }

    #[test]
    fn process_part_1_works() {
        let result = process_part_1(include_str!("../data/test_input.txt"));
        assert_eq!(result, 31);
    }
    #[test]
    fn process_part_2_works() {
        let result = process_part_2(include_str!("../data/test_input.txt"));
        assert_eq!(result, 29);
    }
}

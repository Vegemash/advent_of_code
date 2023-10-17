#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Coords {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Record {
    sensor: Coords,
    beacon: Coords,
}

pub fn process_part_1(input: &str) -> String {
    input.to_string()
}

pub fn process_part_2(input: &str) -> String {
    input.to_string()
}

fn gen_map(input: &str) -> String {
    let map = get_map(input);
    let (minx, miny, maxx, maxy) = get_extents(&map);
    let mut output = "".to_string();
    for y in miny..=maxy {
        for x in minx..=maxx {
            let pos = Coords { x, y };
            if map.iter().any(|r| r.sensor == pos) {
                output.push_str("S");
            } else if map.iter().any(|r| r.beacon == pos) {
                output.push_str("B");
            } else {
                output.push_str(".");
            }
        }
        if y != maxy {
            output.push_str("\n")
        }
    }
    output
}

fn get_map(input: &str) -> Vec<Record> {
    input
        .split_terminator("\n")
        .map(|x| parse_sensor(x))
        .collect()
}

fn get_extents(map: &Vec<Record>) -> (i32, i32, i32, i32) {
    let mut xmin: Option<i32> = None;
    let mut xmax: Option<i32> = None;
    let mut ymin: Option<i32> = None;
    let mut ymax: Option<i32> = None;

    for record in map.iter() {
        if xmin.is_none() || record.sensor.x < xmin.unwrap() {
            xmin = Some(record.sensor.x);
        }
        if xmin.is_none() || record.beacon.x < xmin.unwrap() {
            xmin = Some(record.beacon.x);
        }
        if xmax.is_none() || record.sensor.x > xmax.unwrap() {
            xmax = Some(record.sensor.x);
        }
        if xmax.is_none() || record.beacon.x > xmax.unwrap() {
            xmax = Some(record.beacon.x);
        }
        if ymin.is_none() || record.sensor.y < ymin.unwrap() {
            ymin = Some(record.sensor.y);
        }
        if ymin.is_none() || record.beacon.y < ymin.unwrap() {
            ymin = Some(record.beacon.y);
        }
        if ymax.is_none() || record.sensor.y > ymax.unwrap() {
            ymax = Some(record.sensor.y);
        }
        if ymax.is_none() || record.beacon.y > ymax.unwrap() {
            ymax = Some(record.beacon.y);
        }
    }

    if xmin.is_none() {
        xmin = Some(0);
    }
    (xmin.unwrap(), ymin.unwrap(), xmax.unwrap(), ymax.unwrap())
}

fn coord_parser(s: &str) -> Coords {
    let cords = s.split_once("x").unwrap().1;
    let (xpart, ypart) = cords.split_once(',').unwrap();
    Coords {
        x: xpart.split_once("=").unwrap().1.parse().unwrap(),
        y: ypart.split_once("=").unwrap().1.parse().unwrap(),
    }
}

fn parse_sensor(input: &str) -> Record {
    let (sensor, beacon) = input.split_once(":").unwrap();

    Record {
        sensor: coord_parser(sensor),
        beacon: coord_parser(beacon),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_extents() {
        assert_eq!(
            get_extents(&get_map(include_str!("../data/test_input"))),
            (-2, 0, 25, 22)
        );
    }
    #[test]
    fn test_sensor_parse() {
        assert_eq!(
            parse_sensor("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            Record {
                sensor: Coords { x: 2, y: 18 },
                beacon: Coords { x: -2, y: 15 },
            }
        );
    }
    #[test]
    fn test_map_generation() {
        let result = gen_map(include_str!("../data/test_input"));
        assert_eq!(
            result,
            "\
....S.......................
......................S.....
...............S............
................SB..........
............................
............................
............................
..........S.......S.........
............................
............................
....B.......................
..S.........................
............................
............................
..............S.......S.....
B...........................
...........SB...............
................S..........B
....S.......................
............................
............S......S........
............................
.......................B...."
        );
    }

    #[test]
    #[ignore]
    fn process_part_2_works() {
        let result = process_part_2(
            "
",
        );
        assert_eq!(result, "MCD");
    }
}

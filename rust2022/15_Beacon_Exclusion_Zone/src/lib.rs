use std::collections::HashSet;

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
    count_empty(&get_map(input), 2000000).to_string()
}

pub fn process_part_2(input: &str) -> String {
    get_empty_freq(&get_map(input), 4_000_000).to_string()
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
        let dist = man_dist(&record.sensor, &record.beacon);
        if xmin.is_none() || (record.sensor.x - dist) < xmin.unwrap() {
            xmin = Some(record.sensor.x - dist);
        }
        if xmin.is_none() || record.beacon.x < xmin.unwrap() {
            xmin = Some(record.beacon.x);
        }
        if xmax.is_none() || (record.sensor.x + dist) > xmax.unwrap() {
            xmax = Some(record.sensor.x + dist);
        }
        if xmax.is_none() || record.beacon.x > xmax.unwrap() {
            xmax = Some(record.beacon.x);
        }
        if ymin.is_none() || (record.sensor.y - dist) < ymin.unwrap() {
            ymin = Some(record.sensor.y - dist);
        }
        if ymin.is_none() || record.beacon.y < ymin.unwrap() {
            ymin = Some(record.beacon.y);
        }
        if ymax.is_none() || (record.sensor.y + dist) > ymax.unwrap() {
            ymax = Some(record.sensor.y + dist);
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

fn man_dist(a: &Coords, b: &Coords) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn count_empty(map: &Vec<Record>, row: i32) -> usize {
    let (xmin, _, xmax, _) = get_extents(map);
    let mut cells = vec![];
    let mut empty_count = 0;
    let known_beacons = map
        .iter()
        .map(|r| r.beacon.clone())
        .collect::<HashSet<Coords>>();

    for x in xmin..=xmax {
        cells.push(Coords { x, y: row });
    }

    for cell in cells.iter().filter(|c| known_beacons.get(&c).is_none()) {
        for record in map.iter() {
            let dist = man_dist(&record.sensor, &record.beacon);
            if man_dist(&record.sensor, &cell) <= dist {
                empty_count += 1;
                break;
            }
        }
    }

    empty_count
}

#[allow(dead_code)]
fn get_empty_freq(map: &Vec<Record>, max_extent: i32) -> isize {
    let mut x;
    let known_beacons = map
        .iter()
        .map(|r| r.beacon.clone())
        .collect::<HashSet<Coords>>();
    for y in 0..=max_extent {
        x = 0;
        while x <= max_extent {
            let mut skip = false;
            let cell = Coords { x, y };

            for record in map.iter() {
                let sensor_range = man_dist(&record.sensor, &record.beacon);
                if man_dist(&record.sensor, &cell) <= sensor_range {
                    skip = true;
                    // We could end up jumping to part way through a sensor area
                    // so check here if we are on the left of the sensor
                    if x <= record.sensor.x {
                        // Skip to the other side of this sensor area
                        x += (record.sensor.x - x).abs() * 2;
                    } else {
                        x += sensor_range - man_dist(&record.sensor, &cell);
                    }
                    break;
                }
            }

            if !skip  && known_beacons.get(&cell).is_none(){
                println!("{} {}", x, y);
                return x as isize * 4_000_000 + y as isize ;
            }
            x += 1;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let map = get_map(include_str!("../data/test_input"));
        assert_eq!(count_empty(&map, 10), 26);
    }
    #[test]
    fn test_map_extents() {
        assert_eq!(
            get_extents(&get_map(include_str!("../data/test_input"))),
            (-8, -10, 28, 26)
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
    fn test_get_empty_freq() {
        let map = get_map(include_str!("../data/test_input"));
        assert_eq!(get_empty_freq(&map, 20), 56000011);
    }
}

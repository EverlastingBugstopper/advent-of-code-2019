mod input;
mod map;
mod wire;
use input::INPUT;
use map::Map;
use wire::direction::Direction;
use wire::Wire;

pub fn solution() -> Result<(), failure::Error> {
    let wires = INPUT.iter().map(|x| Wire::new(x).unwrap()).collect();
    let mut map: Map = Map::new(30000);
    process(wires, &mut map)?;
    Ok(())
}

fn process(wires: Vec<Wire>, map: &mut Map) -> Result<(), failure::Error> {
    let mut intersections: Vec<(usize, usize)> = Vec::new();
    for (wire_num, wire) in wires.iter().enumerate() {
        let mut x_pointer = map.center;
        let mut y_pointer = map.center;
        for movement in &wire.path.movements {
            match movement.direction {
                Direction::Right => {
                    for x in x_pointer..movement.distance + x_pointer {
                        let is_intersection = map.cross(x, y_pointer, wire_num);
                        if is_intersection {
                            intersections.push((x, y_pointer));
                        }
                    }
                    x_pointer += movement.distance;
                }
                Direction::Left => {
                    for x in x_pointer - movement.distance..x_pointer {
                        let is_intersection = map.cross(x, y_pointer, wire_num);
                        if is_intersection {
                            intersections.push((x, y_pointer));
                        }
                    }
                    x_pointer -= movement.distance;
                }
                Direction::Down => {
                    for y in y_pointer..movement.distance + y_pointer {
                        let is_intersection = map.cross(x_pointer, y, wire_num);
                        if is_intersection {
                            intersections.push((x_pointer, y));
                        }
                    }
                    y_pointer += movement.distance;
                }
                Direction::Up => {
                    for y in y_pointer - movement.distance..y_pointer {
                        let is_intersection = map.cross(x_pointer, y, wire_num);
                        if is_intersection {
                            intersections.push((x_pointer, y));
                        }
                    }
                    y_pointer -= movement.distance;
                }
            }
            let is_intersection = map.cross(x_pointer, y_pointer, wire_num);
            if is_intersection {
                intersections.push((x_pointer, y_pointer));
            }
        }
    }
    let mut closest = usize::max_value();
    for (x, y) in intersections {
        let x_distance = if x > map.center {
            x - map.center
        } else {
            map.center - x
        };
        let y_distance = if y > map.center {
            y - map.center
        } else {
            map.center - y
        };
        let distance = x_distance + y_distance;
        if distance < closest {
            closest = distance
        }
    }
    println!("{}", closest);
    Ok(())
}

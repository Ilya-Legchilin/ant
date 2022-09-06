use std::collections::HashSet;

const INITIAL_X: i16 = 1000;
const INITIAL_Y: i16 = 1000;
const DIGITS_SUM_LIMIT: u16 = 25;

#[derive(Hash, PartialEq, Eq, Clone)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn go_up(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn go_down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn go_left(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn go_right(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
}

fn digits_sum(number: i16) -> u16 {
    let mut number = match number < 0 {
        true => (number * (-1)) as u16,
        _ => number as u16,
    };
    let mut result = 0;
    while number > 0 {
        result += number % 10;
        number /= 10;
    }
    result
}

fn is_coordinate_available(point: &Point) -> bool {
    (digits_sum(point.x) + digits_sum(point.y)) <= DIGITS_SUM_LIMIT
}

fn count_available_coordinates(point: Point, used_points: &mut HashSet<Point>) -> u32 {
    let mut result = 0;
    if used_points.contains(&point) {
        return result;
    }
    if is_coordinate_available(&point) {
        used_points.insert(point.clone());
        result += 1;
    } else {
        return result;
    }
    if is_coordinate_available(&point.go_up()) {
        result += count_available_coordinates(point.go_up(), used_points);
    }
    if is_coordinate_available(&point.go_down()) {
        result += count_available_coordinates(point.go_down(), used_points);
    }
    if is_coordinate_available(&point.go_left()) {
        result += count_available_coordinates(point.go_left(), used_points);
    }
    if is_coordinate_available(&point.go_right()) {
        result += count_available_coordinates(point.go_right(), used_points);
    }
    return result;
}

fn main() {
    let mut used_points = HashSet::new();
    let result = count_available_coordinates(
        Point {
            x: INITIAL_X,
            y: INITIAL_Y,
        },
        &mut used_points,
    );
    println!("result = {result}");
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn digits_sum_test() {
        assert!(digits_sum(0) == 0);
        assert!(digits_sum(1) == 1);
        assert!(digits_sum(2) == 2);
        assert!(digits_sum(19) == 10);
        assert!(digits_sum(987) == 24);
        assert!(digits_sum(5432) == 14);
        assert!(digits_sum(12345) == 15);
    }

    #[test]
    fn coordinates_availability_test() {
        assert!(is_coordinate_available(&Point { x: 1000, y: 1000 }));
        assert!(!is_coordinate_available(&Point { x: 99, y: 99 }));
        assert!(is_coordinate_available(&Point { x: 99, y: 34 }));
        assert!(!is_coordinate_available(&Point { x: 99, y: 35 }));
    }
}

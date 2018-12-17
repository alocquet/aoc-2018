use utils;
use utils::Point;
use utils::*;

pub fn run() -> (Point, Point) {
    // parse input
    let input = utils::read_file("inputs/day13.txt");
    let (mut carts, map) = parse(&input);
    let crashes = compute_first_crash(&mut carts, &map);

    let first_crash = crashes.first().expect("should have at least one crash");
    let valid_cart = carts.iter().filter(|cart| !cart.crashed).last().expect("should persist a valid cart");
    (*first_crash, valid_cart.position)
}

fn compute_first_crash(carts: &mut Vec<Cart>, map: &[Vec<char>]) -> Vec<Point> {
    let mut crash_queue = vec!();

    while carts.iter().filter(|cart| !cart.crashed).count() > 1 {
        carts.sort_by_key(|cart| (cart.position.y, cart.position.x));

        for id_cart in 0..carts.len() {
            if !carts[id_cart].crashed {
                { carts[id_cart].move_next(map); }
                let cart_position = carts[id_cart].position;
                let mut crashes: Vec<&mut Cart> = carts.iter_mut().filter(|other| !other.crashed && other.position == cart_position).collect();

                if crashes.len() > 1 {
                    crash_queue.push(cart_position);
                    crashes.iter_mut().for_each(|cart| cart.crashed = true);
                }
            }
        }
    }
    crash_queue
}

#[derive(Debug)]
struct Cart {
    position: Point,
    direction: Point,
    nb_intersections: usize,
    crashed: bool,
}

impl Cart {
    fn move_next(&mut self, map: &[Vec<char>]) {
        let cell = map[self.position.y as usize][self.position.x as usize];
        self.direction = match cell {
            '/' => Point { x: -self.direction.y, y: -self.direction.x },
            '\\' => Point { x: self.direction.y, y: self.direction.x },
            '+' => {
                let next_direction = match self.nb_intersections {
                    0 => Point { x: self.direction.y, y: -self.direction.x },
                    1 => self.direction,
                    _ => Point { x: -self.direction.y, y: self.direction.x },
                };
                self.nb_intersections = (self.nb_intersections + 1) % 3;
                next_direction
            }
            _ => self.direction,
        };
        self.position += self.direction;
    }
}

fn parse(input: &str) -> (Vec<Cart>, Vec<Vec<char>>) {
    let mut cars = vec!();
    let map = input.lines().enumerate().map(|(y, line)| line.chars().enumerate().map(|(x, cell)| {
        let direction = match cell {
            'v' => Some(SOUTH),
            '^' => Some(NORTH),
            '>' => Some(EAST),
            '<' => Some(WEST),
            _ => None
        };
        if direction.is_some() {
            cars.push(Cart { position: Point { x: x as isize, y: y as isize }, direction: direction.unwrap(), nb_intersections: 0, crashed: false });
        }
        match cell {
            'v' | '^' => '|',
            '>' | '<' => '-',
            _ => cell
        }
    }).collect()).collect();
    (cars, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "/->-\\
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/";

    #[test]
    fn move_cart() {
        let (mut carts, map) = parse(INPUT1);
        assert_eq!(compute_first_crash(&mut carts, &map).first().unwrap().clone(), Point { x: 7, y: 3 });
    }

    #[test]
    fn input() {
        assert_eq!(run(), (Point { x: 83, y: 106 }, Point { x: 132, y: 26 }));
    }
}

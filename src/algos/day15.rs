use utils::Point;
use utils::*;
use std::collections::VecDeque;

pub fn run() -> (isize, isize) {
    let step1 = run_game(3);

    for power in 4.. {
        let result = run_game(power);
        if result.1 {
            return (step1.0, result.0);
        }
    }

    (step1.0, 0)
}


fn run_game(elf_power: isize) -> (isize, bool) {
    let input = &read_file("inputs/day15.txt");
    let (mut users, map) = parse(input);

    let mut nb_gobelin = count_users(&users, 'G');
    let mut nb_elves = count_users(&users, 'E');

    let mut i = 0;
    while nb_gobelin != 0 && nb_elves != 0 {
        users.sort_by_key(|user| user.position);
        let mut break_before_ends = false;

        for idx in 0..users.len() {
            if users[idx].heatlh > 0 {
                if nb_elves == 0 || nb_gobelin == 0 {
                    break_before_ends = true;
                    break;
                }

                let user_type = users[idx].user_type;
                let user_position = users[idx].position;
                let power = if user_type == 'E' { elf_power } else { 3 };
                if !attack(&user_position, user_type, &mut users, power) {
                    // move
                    let shortest_path = shortest_path(&users[idx], &map, &users);
                    if shortest_path.is_some() {
                        users[idx].position = shortest_path.unwrap()[1];
                    }
                    // attack
                    let user_position = users[idx].position;
                    attack(&user_position, user_type, &mut users, power);
                }

                nb_gobelin = count_users(&users, 'G');
                nb_elves = count_users(&users, 'E');
            }
        }
        if !break_before_ends {
            i += 1;
        }
    }

    let res: isize = users.iter().map(|user| if user.heatlh < 0 { 0 } else { user.heatlh }).sum();
    let elf_dead = users.iter().find(|user| user.heatlh <= 0 && user.user_type == 'E');

    (i * res, elf_dead.is_none())
}

fn attack(user_position: &Point, user_type: char, users: &mut Vec<User>, power: isize) -> bool {
    let neightboor_positions = [*user_position + NORTH, *user_position + WEST, *user_position + EAST, *user_position + SOUTH];
    let mut enemies: Vec<&mut User> = users.iter_mut().filter(|user| user.heatlh > 0 && user.user_type != user_type && neightboor_positions.contains(&user.position)).collect();

    if !enemies.is_empty() {
        enemies.sort_by(|user1, user2| {
            if user1.heatlh == user2.heatlh {
                user1.position.cmp(&user2.position)
            } else {
                user1.heatlh.cmp(&user2.heatlh)
            }
        });
        enemies[0].heatlh -= power;
        return true;
    }

    false
}

fn count_users(users: &[User], user_type: char) -> usize {
    users.iter().filter(|user| user.user_type == user_type && user.heatlh > 0).count()
}

fn shortest_path(user: &User, map: &[Vec<char>], users: &[User]) -> Option<Path> {
    let mut queue = VecDeque::new();
    queue.push_back(vec!(user.position));
    let mut visited = vec!(user.position);
    while !queue.is_empty() {
        let current_path = queue.pop_front().unwrap();
        let current = *current_path.clone().last().unwrap();
        for next in &[current + NORTH, current + WEST, current + EAST, current + SOUTH] {
            if users.iter().any(|u| u.heatlh > 0 && u.user_type != user.user_type && u.position == *next) {
                current_path.clone().push(next.clone());
                return Some(current_path);
            }
            if !visited.contains(&next) {
                let mut next_path = current_path.clone();
                next_path.push(next.clone());
                if map[next.y as usize][next.x as usize] == '.' && users.iter().find(|u| u.heatlh > 0 && u.position == *next).is_none() {
                    queue.push_back(next_path);
                    visited.push(next.clone());
                }
            }
        }
    }
    None
}

type Map = Vec<Vec<char>>;
type Path = Vec<Point>;

#[derive(Debug)]
struct User {
    position: Point,
    user_type: char,
    heatlh: isize,
}

fn parse(input: &str) -> (Vec<User>, Map) {
    let mut users = vec!();
    let map = input.lines().enumerate().map(|(y, line)| {
        line.to_owned().chars().enumerate().map(|(x, car)| {
            if car == 'G' || car == 'E' {
                users.push(User { position: Point { x: x as isize, y: y as isize }, user_type: car, heatlh: 200 });
                '.'
            } else {
                car
            }
        }).collect()
    }).collect();
    (users, map)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        assert_eq!(run(), (213692, 52688));
    }
}

pub mod model;

use utils::read_file;
use self::model::NanoBot;
use algos::day23::model::ZPoint;

pub fn run() -> (usize, usize) {
    let input = &read_file("inputs/day23.txt");
    run_from_input(input)
}

fn run_from_input(input: &str) -> (usize, usize) {
    let nanobots = parse(input);

    (step1(&nanobots), step2(&nanobots))
}

fn step1(nanobots: &[NanoBot]) -> usize {
    let strongest = nanobots.iter().max_by_key(|nanobot| nanobot.radius).expect("sould have at least one nanobot");
    nanobots.iter().filter(|nanobot| strongest.position.dist_from(nanobot.position) <= strongest.radius).count()
}

fn step2(nanobots: &[NanoBot]) -> usize {
    let mut min_x = nanobots.iter().map(|a| a.position.x).min().expect("sould have at least one nanobot");
    let mut max_x = nanobots.iter().map(|a| a.position.x).max().expect("sould have at least one nanobot");
    let mut min_y = nanobots.iter().map(|a| a.position.y).min().expect("sould have at least one nanobot");
    let mut max_y = nanobots.iter().map(|a| a.position.y).max().expect("sould have at least one nanobot");
    let mut min_z = nanobots.iter().map(|a| a.position.z).min().expect("sould have at least one nanobot");
    let mut max_z = nanobots.iter().map(|a| a.position.z).max().expect("sould have at least one nanobot");

    // arbitrary value
    let mut range = 1_000_000_000;

    let zero = ZPoint { x: 0, y: 0, z: 0 };

    loop {
        let mut max_count = 0;
        let mut best = zero;
        let mut best_val = 0;

        for x in (min_x..=max_x).step_by(range as usize) {
            for y in (min_y..=max_y).step_by(range as usize) {
                for z in (min_z..=max_z).step_by(range as usize) {
                    let current = ZPoint { x, y, z };
                    let count = nanobots
                        .iter()
                        .filter(|bot| (current.dist_from(bot.position) as isize - bot.radius as isize) / range <= 0)
                        .count();
                    if count > max_count {
                        max_count = count;
                        best_val = current.dist_from(zero);
                        best = current;
                    }
                }
            }
        }

        if range == 1 {
            return best_val as usize;
        }

        min_x = best.x - range;
        max_x = best.x + range;
        min_y = best.y - range;
        max_y = best.y + range;
        min_z = best.z - range;
        max_z = best.z + range;

        range /= 2;
    }
}


fn parse(input: &str) -> Vec<NanoBot> {
    input.lines().map(|line| NanoBot::new(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

    const INPUT_STEP2: &str = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";

    #[test]
    fn example_step1() {
        let nanobots = parse(INPUT);
        assert_eq!(step1(&nanobots), 7);
    }

    #[test]
    fn example_step2() {
        let nanobots = parse(INPUT_STEP2);
        assert_eq!(step2(&nanobots), 36);
    }

    #[test]
    fn input() {
        assert_eq!(run(), (442, 100985898));
    }
}

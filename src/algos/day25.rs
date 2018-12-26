use utils::read_file;

pub fn run() -> usize {
    let input = &read_file("inputs/day25.txt");

    step1(input)
}

fn step1(input: &str) -> usize {
    let mut stars = parse(&input);
    let mut constellations_idx = 0;
    let mut count = 0;
    for star_id in 0..stars.len() {
        for other_star_id in 0..star_id {
            if stars[star_id].distance_from(&stars[other_star_id]) <= 3 {
                match stars[star_id].constellation_id {
                    Some(constellation_id) => {
                        if stars[other_star_id].constellation_id.unwrap() != constellation_id {
                            // merge constellations
                            let constellation_to_merge = stars[other_star_id].constellation_id.unwrap();
                            for star_to_merge in &mut stars {
                                if star_to_merge.constellation_id.is_some() && star_to_merge.constellation_id.unwrap() == constellation_to_merge {
                                    star_to_merge.constellation_id = Some(constellation_id);
                                }
                            }
                            count -= 1;
                        }
                    }
                    None => stars[star_id].constellation_id = Some(stars[other_star_id].constellation_id.expect("should have a constellation id"))
                }
            }
        }
        if stars[star_id].constellation_id.is_none() {
            stars[star_id].constellation_id = Some(constellations_idx);
            constellations_idx += 1;
            count += 1;
        }
    }

    count
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Constellation {
    stars: Vec<Star>
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Star {
    coordinates: Vec<isize>,
    constellation_id: Option<usize>,
}

impl Star {
    fn distance_from(&self, other: &Star) -> usize {
        (0..4).map(|i| (self.coordinates[i] - other.coordinates[i]).abs() as usize).sum()
    }
}

fn parse(input: &str) -> Vec<Star> {
    input.lines()
        .map(|line|
            Star {
                coordinates: line.split(',')
                    .map(|coord| coord.parse::<isize>().expect("bas coordinate format"))
                    .collect(),
                constellation_id: None,
            }
        ).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE_1: &str = "0,0,0,0\n3,0,0,0\n0,3,0,0\n0,0,3,0\n0,0,0,3\n0,0,0,6\n9,0,0,0\n12,0,0,0";
    const INPUT_EXAMPLE_2: &str = "-1,2,2,0\n0,0,2,-2\n0,0,0,-2\n-1,2,0,0\n-2,-2,-2,2\n3,0,2,-1\n-1,3,2,2\n-1,0,-1,0\n0,2,1,-2\n3,0,0,0";
    const INPUT_EXAMPLE_3: &str = "1,-1,0,1\n2,0,-1,0\n3,2,-1,0\n0,0,3,1\n0,0,-1,-1\n2,3,-2,0\n-2,2,0,0\n2,-2,0,-1\n1,-1,0,-1\n3,2,0,2";
    const INPUT_EXAMPLE_4: &str = "1,-1,-1,-2\n-2,-2,0,1\n0,2,1,3\n-2,3,-2,1\n0,2,3,-2\n-1,-1,1,-2\n0,-2,-1,0\n-2,2,3,-1\n1,2,2,0\n-1,-2,0,-2";

    #[test]
    fn example_step1() {
        assert_eq!(step1(INPUT_EXAMPLE_1), 2);
        assert_eq!(step1(INPUT_EXAMPLE_2), 4);
        assert_eq!(step1(INPUT_EXAMPLE_3), 3);
        assert_eq!(step1(INPUT_EXAMPLE_4), 8);
    }

    #[test]
    fn input() {
        assert_eq!(run(), 373);
    }
}


use std::collections::HashMap;
use utils;

pub fn run() -> (i32, String) {
    // parse input
    let input = utils::read_file("inputs/day02.txt");
    let ids = input.lines().collect::<Vec<&str>>();

    (checksum(&ids), find_correct_box_id(&ids).expect("No correct box id found"))
}

fn checksum(ids: &[&str]) -> i32 {
    let (nb_dbl, nb_tri) = ids.iter().fold((0, 0), |(acc_dbl, acc_tri), id| {
        let (id_dbl, id_tri) = match_id(id);
        (acc_dbl + id_dbl, acc_tri + id_tri)
    });
    nb_dbl * nb_tri
}

fn match_id(id: &str) -> (i32, i32) {
    let mut visited = HashMap::new();
    let mut double = 0;
    let mut triple = 0;

    for character in id.chars() {
        let visited_count = match visited.get(&character) {
            Some(count) => count + 1,
            None => 1,
        };
        visited.insert(character, visited_count);
    }

    for visit in visited.values() {
        if *visit == 2 {
            double = 1;
        } else if *visit == 3 {
            triple = 1;
        }
    }

    (double, triple)
}

fn find_correct_box_id(ids: &[&str]) -> Option<String> {
    for (idx, id) in ids.iter().enumerate() {
        for other in &ids[idx + 1..] {
            let sim = similarities(id, other);
            if id.len() == sim.len() + 1 {
                return Some(sim);
            }
        }
    }
    None
}

fn similarities(a: &str, b: &str) -> String {
    let mut sims = Vec::new();
    for (idx, char_a) in a.chars().enumerate() {
        let char_b = b.get(idx..=idx).unwrap().chars().next().unwrap();
        if char_a == char_b {
            sims.push(char_a);
        }
    }
    sims.iter().collect()
}

#[cfg(test)]
mod tests_step1 {
    use super::*;

    #[test]
    fn abcde() {
        assert_eq!(match_id("abcdef"), (0, 0));
    }

    #[test]
    fn bababc() {
        assert_eq!(match_id("bababc"), (1, 1));
    }

    #[test]
    fn abbcde() {
        assert_eq!(match_id("abbcde"), (1, 0));
    }

    #[test]
    fn abcccd() {
        assert_eq!(match_id("abcccd"), (0, 1));
    }

    #[test]
    fn aabcdd() {
        assert_eq!(match_id("aabcdd"), (1, 0));
    }

    #[test]
    fn abcdee() {
        assert_eq!(match_id("abcdee"), (1, 0));
    }

    #[test]
    fn ababab() {
        assert_eq!(match_id("ababab"), (0, 1));
    }

    #[test]
    fn checksum_example() {
        assert_eq!(checksum(&vec!("abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab")), 12);
    }
}

#[cfg(test)]
mod tests_step2 {
    use super::*;

    #[test]
    fn sim() {
        assert_eq!(similarities("abcde", "axcye"), "ace");
    }

    #[test]
    fn similarities_between_good_ids() {
        assert_eq!(find_correct_box_id(&vec!("abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz")).unwrap(), "fgij".to_owned());
    }

    #[test]
    fn no_good_ids() {
        assert_eq!(find_correct_box_id(&vec!("abcde", "fghij")), None);
    }
}

#[cfg(test)]
mod tests_input {
    use super::*;

    #[test]
    fn input() {
        assert_eq!(run(), (7904, "wugbihckpoymcpaxefotvdzns".to_owned()));
    }
}
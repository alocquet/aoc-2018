use regex::Regex;
use rayon::prelude::*;
use utils;

#[cfg_attr(tarpaulin, skip)]
pub fn run() -> (usize, usize) {
    let input = utils::read_file("inputs/day05.txt");

    (compute_stable_polymer(input.clone()).len(), compute_min_polymer_size(&input))
}

fn compute_stable_polymer(input: String) -> String {
    let mut polymer = input;
    let mut length = 0;
    let re = Regex::new(r"(aA|Aa|bB|Bb|cC|Cc|dD|Dd|eE|Ee|fF|Ff|gG|Gg|hH|Hh|iI|Ii|jJ|Jj|kK|Kk|lL|Ll|mM|Mm|nN|Nn|oO|Oo|pP|Pp|qQ|Qq|rR|Rr|sS|Ss|tT|Tt|uU|Uu|vV|Vv|wW|Ww|xX|Xx|yY|Yy|zZ|Zz)").unwrap();

    while polymer.len() != length {
        length = polymer.len();
        polymer = re.replace_all(&polymer, "").into();
    }

    polymer
}

fn compute_min_polymer_size(input: &str) -> usize {
    ((b'a')..(b'{'))
        .into_par_iter()
        .map(|unit| compute_stable_polymer(input.replace(unit as char, "").replace((unit - 32) as char, "")).len())
        .min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_step1() {
        assert_eq!(compute_stable_polymer("dabAcCaCBAcCcaDA".to_owned()).len(), 10);
    }

    #[test]
    fn example_step2() {
        assert_eq!(compute_min_polymer_size("dabAcCaCBAcCcaDA"), 4);
    }

    #[test]
    fn input() {
        // Disable this test because it is too long in debug mode, and release mode can't be activated on travis if we want code coverage
        // assert_eq!(run(), (9900, 4992));
    }
}


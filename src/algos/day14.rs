pub fn run() -> (String, usize) {
    (step1(652_601), step2(&[6, 5, 2, 6, 0, 1]))
}

fn compute_recipes<F>(continue_predicate: F) -> Vec<usize> where F: Fn(usize, &[usize]) -> bool {
    let mut recipes = vec!(3, 7);
    let mut elf_1 = 0;
    let mut elf_2 = 1;

    let mut idx = 0;
    while continue_predicate(idx, &recipes) {
        idx += 1;
        let sum = recipes[elf_1] + recipes[elf_2];
        if sum < 10 {
            recipes.push(sum);
        } else {
            recipes.push(1);
            recipes.push(sum % 10);
        }
        elf_1 = (elf_1 + recipes[elf_1] + 1) % recipes.len();
        elf_2 = (elf_2 + recipes[elf_2] + 1) % recipes.len();
    }
    recipes
}

fn step1(input: usize) -> String {
    let recipes = compute_recipes(|idx, _| idx < input + 10);
    recipes[input..input + 10].iter().map(|&rec| rec.to_string()).collect::<Vec<String>>().join("")
}

fn step2(input: &[usize]) -> usize {
    let recipes = compute_recipes(|_, rec| {
        (rec.len() < input.len() || rec[rec.len() - input.len()..] != *input)
            && (rec.len() < input.len() + 1 || rec[rec.len() - input.len() - 1..rec.len() - 1] != *input)
    });

    if recipes[recipes.len() - input.len()..] == *input {
        recipes.len() - input.len()
    } else {
        recipes.len() - input.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_examples() {
        assert_eq!(step1(9), "5158916779");
        assert_eq!(step1(5), "0124515891");
        assert_eq!(step1(18), "9251071085");
        assert_eq!(step1(2018), "5941429882");
    }

    #[test]
    fn step2_examples() {
        assert_eq!(step2(&[5, 1, 5, 8, 9]), 9);
        assert_eq!(step2(&[0, 1, 2, 4, 5]), 5);
        assert_eq!(step2(&[9, 2, 5, 1, 0]), 18);
        assert_eq!(step2(&[5, 9, 4, 1, 4]), 2018);
    }

    #[test]
    fn input() {
        assert_eq!(run(), ("1221283494".to_owned(), 20261485));
    }
}

use utils;

pub fn run() -> (usize, usize) {
    let input = utils::read_file("inputs/day08.txt");
    let root = parse(&parse_str(&input)).0;

    (sum_metadata(&root), root.value())
}

fn sum_metadata(root: &Node) -> usize {
    root.metadata.iter().sum::<usize>() + root.children.iter().map(|child| sum_metadata(child)).sum::<usize>()
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum::<usize>()
        } else {
            // metadata is indexes
            let mut cache = vec!(None; self.children.len());
            let mut sum = 0;
            for &idx in &self.metadata {
                let child = idx - 1;
                if child < self.children.len() {
                    let child_sum = cache[child];
                    if child_sum.is_some() {
                        sum += child_sum.unwrap();
                    } else {
                        let child_value = self.children[child].value();
                        cache[child] = Some(child_value);
                        sum += child_value;
                    }
                }
            }
            sum
        }
    }
}

fn parse(input: &[usize]) -> (Node, &[usize]) {
    let mut nb_children = input[0];
    let mut children = vec!();
    let nb_metadata = input[1];

    let mut input = &input[2..];

    while nb_children > 0 {
        nb_children -= 1;
        let (child, next) = parse(&input);
        children.push(child);
        input = next;
    }

    (Node { children, metadata: input[..nb_metadata].to_owned() }, &input[nb_metadata..])
}

fn parse_str(input: &str) -> Vec<usize> {
    input.split(' ').map(|item| item.parse::<usize>().expect("an item is not a number")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn parse_example_input_string() {
        assert_eq!(parse_str(INPUT_EXAMPLE).len(), 16);
    }

    #[test]
    fn example_step1() {
        assert_eq!(sum_metadata(&parse(&parse_str(INPUT_EXAMPLE)).0), 138);
    }

    #[test]
    fn example_step2() {
        assert_eq!(parse(&parse_str(INPUT_EXAMPLE)).0.value(), 66);
    }

    #[test]
    fn input() {
        assert_eq!(run(), (43996, 35189));
    }
}

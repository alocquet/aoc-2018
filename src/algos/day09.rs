use std::collections::VecDeque;

pub fn run() -> (usize, usize) {
    (compute_best_score(473, 70904), compute_best_score(473, 70904*100))
}

fn compute_best_score(nb_players: usize, nb_marbles: usize) -> usize {
    let mut scores = vec!(0; nb_players);
    let mut marbles: VecDeque<usize> = VecDeque::new();
    marbles.push_back(0);

    for current_marble in 1..nb_marbles {
        if current_marble % 23 == 0 {
            (0..7).for_each(|_| mv(&mut marbles, &Direction::BACK));
            let player = current_marble % nb_players;
            scores[player] += current_marble + marbles.pop_front().unwrap();
        } else {
            (0..2).for_each(|_| mv(&mut marbles, &Direction::FORWARD));
            marbles.push_front(current_marble);
        }
    }

    *scores.iter().max().unwrap()
}

enum Direction {
    FORWARD,
    BACK,
}

fn mv(marbles: &mut VecDeque<usize>, direction: &Direction) {
    match direction {
        Direction::FORWARD => {
            let popped = marbles.pop_front().unwrap();
            marbles.push_back(popped);
        }
        Direction::BACK => {
            let popped = marbles.pop_back().unwrap();
            marbles.push_front(popped);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_step1() {
        assert_eq!(compute_best_score(5, 25), 32);
        assert_eq!(compute_best_score(10, 1618), 8317);
        assert_eq!(compute_best_score(13, 7999), 146373);
        assert_eq!(compute_best_score(21, 6111), 54718);
        assert_eq!(compute_best_score(30, 5807), 37305);
    }

    #[test]
    fn input() {
        assert_eq!(run(), (371284, 3038972494));
    }
}

use utils;
use algos::day24::model::Team;
use algos::day24::model::Group;

pub mod model;

pub fn run() -> (usize, usize) {
    run_from_input("inputs/day24.txt")
}

fn run_from_input(file_name: &str) -> (usize, usize) {
    let input = &utils::read_file(file_name);
    let groups = parse(input);
    let (_, step1) = fight(groups.clone());

    let mut boost = 1;
    let step2;
    loop {
        let mut group = groups.clone();
        group
            .iter_mut()
            .filter(|u| u.team == Team::Immune)
            .for_each(|u| u.dmg += boost);
        let (team, res) = fight(group);
        if team.is_some() && team.unwrap() == Team::Immune {
            step2 = Some(res);
            break;
        }
        boost += 1;
    }
    (step1, step2.unwrap())
}

fn fight(mut groups: Vec<Group>) -> (Option<Team>, usize) {
    loop {
        // compute targets
        let targets = compute_targets(&mut groups);

        // attack
        if attack(&mut groups, &targets) == 0 {
            return (None, 0);
        }

        let mut immune_alive = 0;
        let mut infection_alive = 0;
        for group in &groups {
            if group.team == Team::Immune {
                immune_alive += group.units;
            } else {
                infection_alive += group.units;
            }
        }
        if immune_alive == 0 && infection_alive == 0 {
            return (None, 0);
        } else if immune_alive == 0 {
            return (Some(Team::Infect), infection_alive);
        } else if infection_alive == 0 {
            return (Some(Team::Immune), immune_alive);
        }
    }
}

fn compute_targets(groups: &mut Vec<Group>) -> Vec<Option<usize>> {
    let mut targets = vec!(None; groups.len());
    groups.sort_by_key(|v| (v.effective_power(), v.initiative));
    groups.reverse();
    for (attacker_idx, attacker) in groups.iter().enumerate() {
        let mut best_attack = 0;
        for (defender_idx, defender) in groups.iter().enumerate() {
            if attacker.team == defender.team || targets.contains(&Some(defender_idx)) || defender.units == 0 {
                continue;
            }
            if defender.damaged_ammount(&attacker) > best_attack {
                best_attack = defender.damaged_ammount(&attacker);
                targets[attacker_idx] = Some(defender_idx);
            };
        }
    }
    targets
}

fn attack(groups: &mut Vec<Group>, targets: &[Option<usize>]) -> usize {
    // attack
    let mut attackers = (0..groups.len()).collect::<Vec<_>>();
    attackers.sort_by_key(|&idx| groups[idx].initiative);
    attackers.reverse();
    let mut killed = 0;
    for attacker_idx in attackers {
        if groups[attacker_idx].units == 0 {
            continue;
        }
        if let Some(defender_idx) = targets[attacker_idx] {
            let attacker = &groups[attacker_idx].clone();
            killed += groups[defender_idx].incur_attack(&attacker);
        }
    }
    killed
}

fn parse(input: &str) -> Vec<Group> {
    let mut current_team = Team::Immune;
    let mut groups = Vec::new();
    for line in input.lines() {
        match line.trim() {
            "Immune System:" => current_team = Team::Immune,
            "Infection:" => current_team = Team::Infect,
            "" => {}
            _ => groups.push(Group::new(line, current_team)),
        }
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(run_from_input("inputs/day24-example.txt"), (5216, 51));
    }

    #[test]
    fn input() {
        assert_eq!(run(), (13331, 7476));
    }
}


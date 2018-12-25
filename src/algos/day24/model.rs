use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Team {
    Immune,
    Infect,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Group {
    pub team: Team,
    pub units: usize,
    pub hp: usize,
    pub dmg: usize,
    pub factors: HashMap<String, usize>,
    pub dgm_type: String,
    pub initiative: usize,
}

impl Group {
    pub fn new(input: &str, team: Team) -> Self {
        let regex = Regex::new(r"^(\d+) units each with (\d+) hit points (?:\((?:(weak|immune) to ([\w\s,]*))?(?:; )?(?:(weak|immune) to (.*?))?\) )?with an attack that does (\d+) (\w*) damage at initiative (\d+)$").expect("bad regexp format");
        let captures = regex.captures(input).expect("Bad input format");

        let mut factors = HashMap::new();

        if captures.get(3).is_some() {
            match &captures[3] {
                "weak" => { captures[4].split(',').for_each(|val| { factors.insert(val.trim().to_owned(), 2); }) }
                "immune" => { captures[4].split(',').for_each(|val| { factors.insert(val.trim().to_owned(), 0); }) }
                _ => {}
            }
        }
        if captures.get(5).is_some() {
            match &captures[5] {
                "weak" => { captures[6].split(',').for_each(|val| { factors.insert(val.trim().to_owned(), 2); }) }
                "immune" => { captures[6].split(',').for_each(|val| { factors.insert(val.trim().to_owned(), 0); }) }
                _ => {}
            }
        }

        Self {
            team,
            units: captures[1].parse().expect("bad units count format"),
            hp: captures[2].parse().expect("bad hit points format"),
            factors,
            dmg: captures[7].parse().expect("bad damage strength format"),
            dgm_type: captures[8].to_owned(),
            initiative: captures[9].parse().expect("bad initiative format"),
        }
    }

    pub fn damaged_ammount(&self, other: &Group) -> usize {
        other.effective_power() * self.factors.get(&other.dgm_type).unwrap_or(&1)
    }

    pub fn incur_attack(&mut self, other: &Group) -> usize {
        let potentially_killed = self.damaged_ammount(other) / self.hp;
        let killed = if self.units < potentially_killed { self.units } else { potentially_killed };
        self.units -= killed;
        killed
    }

    pub fn effective_power(&self) -> usize {
        self.units * self.dmg
    }
}

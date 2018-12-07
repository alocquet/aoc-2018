use utils;
use std::cell::RefCell;
use std::cell::Ref;
use std::cell::Cell;
use std::collections::HashMap;
use regex::Regex;
use std::rc::Rc;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::iter::FromIterator;

pub fn run() -> (String, usize) {
    let input = utils::read_file("inputs/day07.txt");

    let nodes = parse(&input);
    (get_correct_order(&get_roots(&nodes), &nodes), get_duration(&get_roots(&nodes), &nodes, 5, 60))
}

struct Worker {
    delay: Cell<usize>,
    node: RefCell<Option<String>>,
}

impl Worker {
    fn new() -> Self {
        Worker { delay: Cell::new(0), node: RefCell::new(None) }
    }
    fn get_delay(&self) -> usize { self.delay.get() }
    fn set_delay(&self, delay: usize) { self.delay.set(delay); }
    fn get_node(&self) -> Ref<Option<String>> { self.node.borrow() }
    fn set_node(&self, node: Option<String>) { self.node.replace(node); }
}

fn get_duration(roots: &HashSet<String>, nodes: &HashMap<String, Rc<Node>>, nb_workers: usize, min_duration: usize) -> usize {
    let mut visited: HashSet<String> = HashSet::new();
    let mut accessibles_nodes = vec!();

    let workers: Vec<Worker> = (0..nb_workers).map(|_| Worker::new()).collect();

    roots.iter().for_each(|root| accessibles_nodes.push(root.clone()));
    accessibles_nodes.sort_by(|a, b| b.cmp(a));

    while visited.len() < nodes.len() {
        // extract min worker
        let worker = workers.iter().min_by(|w1, w2| {
            if w1.delay == w2.delay {
                if w1.get_node().is_none() { Ordering::Greater } else { Ordering::Less }
            } else {
                w1.get_delay().cmp(&w2.get_delay())
            }
        }).unwrap();

        if worker.get_node().is_some() {
            // add next nodes to queue
            visited.insert(worker.get_node().clone().unwrap());
            nodes.get(&worker.get_node().clone().unwrap()).unwrap().next.borrow().iter().for_each(|next| {
                if !visited.contains(&next.clone().to_string()) && nodes.get(&next.clone()).unwrap().prev.borrow().iter().find(|prev| !visited.contains(&(*prev).clone().to_string())).is_none() {
                    accessibles_nodes.push(next.clone());
                }
            });
            accessibles_nodes.sort_by(|a, b| b.cmp(a));
            worker.set_node(None);
        }

        let next = accessibles_nodes.pop();

        if next.is_some() {
            let next = next.unwrap();
            worker.set_delay(worker.get_delay() + min_duration + (next.chars().next().unwrap() as usize) - (b'A' as usize) + 1);
            worker.set_node(Some(next.clone()));} else {
            // wait next worker finished
            let second_min = workers.iter()
                .filter(|item| worker.get_delay() < item.get_delay())
                .min_by_key(|item| item.get_delay());
            if second_min.is_some() {
                worker.set_delay(second_min.unwrap().get_delay());
            }
        }
    }

    // Finish
    workers.iter().map(|worker|worker.get_delay()).max().unwrap()
}

fn get_correct_order(roots: &HashSet<String>, nodes: &HashMap<String, Rc<Node>>) -> String {
    let mut visited = HashSet::new();
    let mut accessibles_nodes = vec!();
    roots.iter().for_each(|root| accessibles_nodes.push(root.clone()));
    accessibles_nodes.sort_by(|a, b| b.cmp(a));

    let mut path = String::new();

    while !accessibles_nodes.is_empty() {
        let next = accessibles_nodes.pop().unwrap();
        if !visited.contains(&next) {
            visited.insert(next.clone());
            let node = nodes.get(&next).unwrap();
            node.next.borrow().iter().for_each(|next_node| {
                if nodes.get(&next_node.clone()).unwrap().prev.borrow().iter().find(|prev| !visited.contains(&(*prev).clone().to_string())).is_none() {
                    accessibles_nodes.push(next_node.clone());
                }
            });
            accessibles_nodes.sort_by(|a, b| b.cmp(a));
            path.push_str(&next);
        }
    }

    path
}

fn get_roots(nodes: &HashMap<String, Rc<Node>>) -> HashSet<String> {
    let mut roots: HashSet<&String> = HashSet::from_iter(nodes.keys());
    for node in nodes.values() {
        for next_node in node.next.borrow().iter() {
            roots.remove(next_node);
        }
    }
    roots.iter().map(|root| root.to_string()).collect()
}

#[derive(Debug)]
struct Node {
    name: String,
    next: RefCell<Vec<String>>,
    prev: RefCell<Vec<String>>,
}

fn parse(input: &str) -> HashMap<String, Rc<Node>> {
    let mut nodes: HashMap<String, Rc<Node>> = HashMap::new();
    let regex = Regex::new(r"^Step (.*?) must be finished before step (.*?) can begin.$").unwrap();

    for line in input.lines() {
        let capture = regex.captures(line).unwrap();
        let node_name = capture[1].to_owned();
        let next_node_name = capture[2].to_owned();
        nodes.entry(next_node_name.clone()).or_insert_with(|| Rc::new(Node { name: next_node_name, next: RefCell::new(vec!()), prev: RefCell::new(vec!()) }));
        nodes.entry(node_name.clone()).or_insert_with(|| Rc::new(Node { name: node_name, next: RefCell::new(vec!()), prev: RefCell::new(vec!()) }));

        // TODO check how can I avoid a second get instead of keep two previous call in a variable
        nodes[&capture[1]].next.borrow_mut().push(capture[2].to_owned());
        nodes[&capture[2]].prev.borrow_mut().push(capture[1].to_owned());
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn parser() {
        assert_eq!(parse(EXAMPLE_INPUT).len(), 6);
    }

    #[test]
    fn root_is_c() {
        assert_eq!(get_roots(&parse(EXAMPLE_INPUT)).iter().next().unwrap(), "C");
    }

    #[test]
    fn path_is_cabdfe() {
        let nodes = parse(EXAMPLE_INPUT);
        assert_eq!(get_correct_order(&get_roots(&nodes), &nodes), "CABDFE");
    }

    #[test]
    fn duration_is_15() {
        let nodes = parse(EXAMPLE_INPUT);
        assert_eq!(get_duration(&get_roots(&nodes), &nodes, 2, 0), 15);
    }

    #[test]
    fn input() {
        assert_eq!(run(), ("ABDCJLFMNVQWHIRKTEUXOZSYPG".to_owned(), 896));
    }
}

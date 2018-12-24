use regex::Regex;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ZPoint {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl ZPoint {
    pub fn new(input: &str) -> ZPoint {
        let coords: Vec<isize> = input.split(',').map(|coord| coord.parse().expect("Bad coord format")).collect();
        ZPoint { x: coords[0], y: coords[1], z: coords[2] }
    }
    pub fn dist_from(self, other: ZPoint) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NanoBot {
    pub position: ZPoint,
    pub radius: usize,
}

impl NanoBot {
    pub fn new(input: &str) -> Self {
        lazy_static! {
            static ref regex: Regex = Regex::new(r"^pos=<(.*?)>, r=(\d*)$").expect("Malformed regex");
        }
        let buffer = regex.captures(input).expect("bad format");
        NanoBot { position: ZPoint::new(&buffer[1]), radius: buffer[2].parse().expect("Bad radius format") }
    }
}

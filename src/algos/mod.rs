pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

macro_rules! modules {
    ($($mod:ident),*) => {
        // can't import module in macro if we want autocomplete in IDEA
        // $(pub mod $mod;)*

        pub fn run_day(day: &str) {
            $(
            if stringify!($mod)==day {
                println!("{:?}", self::$mod::run());
            }
            )*
        }
    }
}
modules![
  day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12
];

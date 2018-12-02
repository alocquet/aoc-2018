pub mod day01;
pub mod day02;

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
  day01, day02
];

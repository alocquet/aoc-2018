macro_rules! modules {
    ($($mod:ident),*) => {
        $(pub mod $mod;)*

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

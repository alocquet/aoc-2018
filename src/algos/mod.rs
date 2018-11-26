macro_rules! modules {
    ($($mod:ident),*) => {
        $(pub mod $mod;)*

        pub fn run_day(day: &str, input: Option<&str>) {
            $(
            if stringify!($mod)==day {
                self::$mod::run(input);
            }
            )*
        }
    }
}
modules![
  day01, day02
];

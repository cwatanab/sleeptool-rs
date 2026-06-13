#[derive(Debug, Clone)]
pub struct Cli {
    /// Use hibernate mode instead of sleep
    pub hibernate: bool,

    /// Use legacy input detection method
    pub legacy_input: bool,
}

impl Cli {
    pub fn parse() -> Self {
        let mut hibernate = false;
        let mut legacy_input = false;
        for arg in std::env::args().skip(1) {
            match arg.as_str() {
                "-H" | "--hibernate" => hibernate = true,
                "-O" | "--legacy-input" => legacy_input = true,
                "-h" | "--help" => {
                    println!("SleepTool compatible app in Rust\n\nOptions:\n  -H, --hibernate      Use hibernate mode instead of sleep\n  -O, --legacy-input  Use legacy input detection method\n  -h, --help           Print help");
                    std::process::exit(0);
                }
                "-V" | "--version" => {
                    println!("sleeptool-rs 0.1.0");
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        Self {
            hibernate,
            legacy_input,
        }
    }
}

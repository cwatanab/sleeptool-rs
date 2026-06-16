#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cli {
    /// Use hibernate mode instead of sleep
    pub hibernate: bool,

    /// Use legacy input detection method
    pub legacy_input: bool,
}

impl Cli {
    pub fn parse() -> Self {
        match parse_args(&std::env::args().skip(1).collect::<Vec<_>>()) {
            Ok(cli) => cli,
            Err(msg) => {
                println!("{}", msg);
                std::process::exit(0);
            }
        }
    }
}

pub const HELP_MESSAGE: &str = "SleepTool compatible app in Rust\n\nOptions:\n  -H, --hibernate      Use hibernate mode instead of sleep\n  -O, --legacy-input  Use legacy input detection method\n  -h, --help           Print help";
pub const VERSION_MESSAGE: &str = "sleeptool-rs 0.1.0";

/// コマンドライン引数を解析する。`-h` / `--help` / `-V` / `--version` は
/// 表示メッセージを `Err` で返す（呼び出し側で `println!` して exit する）。
pub fn parse_args(args: &[String]) -> Result<Cli, String> {
    let mut hibernate = false;
    let mut legacy_input = false;
    for arg in args {
        match arg.as_str() {
            "-H" | "--hibernate" => hibernate = true,
            "-O" | "--legacy-input" => legacy_input = true,
            "-h" | "--help" => return Err(HELP_MESSAGE.to_string()),
            "-V" | "--version" => return Err(VERSION_MESSAGE.to_string()),
            _ => {}
        }
    }
    Ok(Cli {
        hibernate,
        legacy_input,
    })
}

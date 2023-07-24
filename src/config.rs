use std::env::Args;

pub enum Mode {
    Server,
    Client,
}

pub struct Config {
    pub mode: Mode,
}

impl Config {
    fn new(mode: Mode) -> Self {
        Config { mode }
    }

    pub fn from_args(args: Args) -> Result<Self, &'static str> {
        let args = args.collect::<Vec<String>>();

        let err: &'static str = "Usage: program --<server|client>";

        if args.len() != 2 {
            return Err(err);
        }

        let mode = match args[1].as_str() {
            "--server" => Mode::Server,
            "--client" => Mode::Client,
            _ => return Err(err),
        };

        return Ok(Config::new(mode));
    }
}

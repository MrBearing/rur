

struct Cli {
    sub_command: String,
}

impl Cli {
    fn from_args() -> Cli{
        let pattern = std::env::args().nth(1).expect("no sub command given");
        let path = std::env::args().nth(2).expect("no pattern given");
        Cli{
            sub_command: pattern,
        }
    }
}

fn main() {
    let args = Cli::from_args();
}



use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Takumi Okamoto <takumi1988okamoto@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap()]
    SendScript(SendScript),
    #[clap()]
    Ds(DashboardServer),
}


#[derive(Clap,Debug)]
struct SendScript{
    input: String,
    input2: String,
}

#[derive(Clap)]
#[clap()]
struct DashboardServer{
    #[clap(subcommand)]
    subsubcmd: DashboardServerSubCommand,
}

#[derive(Clap)]
enum DashboardServerSubCommand{
    #[clap()]
    Play(DSPlay),
    #[clap()]
    Load(DSLoad),
}

#[derive(Clap,Debug)]
struct DSPlay {
    address: String,
}

#[derive(Clap,Debug)]
struct DSLoad {
    address: String,
    file_name: String,
}


fn main() {
    let opts: Opts = Opts::parse();

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::SendScript(t) => println!("{:?}",t),
        SubCommand::Ds(subcmd) => match subcmd.subsubcmd {
            DashboardServerSubCommand::Load(l) => println!("{:?}",l),
            DashboardServerSubCommand::Play(l) => println!("{:?}",l,),
        }
    }

    // more program logic goes here...
}
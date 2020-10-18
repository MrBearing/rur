extern crate rur;

use clap::Clap;
use rur::script;
use rur::dashboard_server;


#[derive(Clap)]
#[clap(version = "1.0", author = "Takumi Okamoto <takumi1988okamoto@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(about = "send script to UR Robot")]
    SendScript(SendScript),
    #[clap(about = "subcommand for controlling dashboard server")]
    Ds(DashboardServer),
}


#[derive(Clap,Debug)]
struct SendScript{
    #[clap(long,default_value="127.0.0.1")]
    host_name:String,
    #[clap(short,long,default_value="30002")]
    port: u32,
    script_file_name: String,
}

#[derive(Clap)]
#[clap()]
struct DashboardServer{
    #[clap(subcommand)]
    sub_subcommand: DashboardServerSubCommand,
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
    #[clap(long,default_value="127.0.0.1")]
    host_name:String,
}

#[derive(Clap,Debug)]
struct DSLoad {
    #[clap(long,default_value="127.0.0.1")]
    host_name:String,
    file_name: String,
}


fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcommand {
        SubCommand::SendScript(s) =>
            script::send(&s.host_name, s.port, &s.script_file_name),
        SubCommand::Ds(subcommand) => match subcommand.sub_subcommand {
            DashboardServerSubCommand::Load(l) =>
                dashboard_server::load(&l.host_name, l.file_name),
            DashboardServerSubCommand::Play(l) =>
                dashboard_server::play(&l.host_name),
        }
    }

}
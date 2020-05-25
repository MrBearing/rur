use std::io::{/* BufReader ,*/ BufWriter};
use super::network::{write_to_tcp_stream,get_connected_tcp_stream};


const DASHBOARD_SERVER_PORT: u32 = 29999;

fn send_command_to_dashboard_server(address: &str, command: &str){
    let tcp_stream = 
        match get_connected_tcp_stream(address,DASHBOARD_SERVER_PORT){
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Fail to open TCP stream. cause: {}",e);
            return;
        }
    };
    let mut buf_writer = BufWriter::new(&tcp_stream);
    let mut command = command.to_string();
    command.push_str("\n");
    write_to_tcp_stream(&mut buf_writer, &command);
}


pub fn play(address: &str) {
    send_command_to_dashboard_server(address, "Play");
}

pub fn load(address: &str, program_name: String){
    // need to validate program_name end with .urp

    let command = format!("Load {}",program_name);
    send_command_to_dashboard_server(address, &command);
}

// pub fn stop(address: &str) {
//     unimplemented!()
// }
// pub fn pause(address: &str) {
//     unimplemented!()
// }
// pub fn quit(address: &str) {
//     unimplemented!()
// }
// pub fn shutdown(address: &str) {
//     unimplemented!()
// }
// pub fn running(address: &str) -> bool{
//     unimplemented!()
// }
// pub enum Robotmode{
//     NO_CONTROLLER,
//     DISCONNECTED,
//     CONFIRM_SAFETY,
//     BOOTING,
//     POWER_OFF,
//     POWER_ON,
//     IDLE,
//     BACKDRIVE,
//     RUNNING,
// }
// pub fn robotmode(address: &str) -> Robotmode {
//     Robotmode::NO_CONTROLLER
// }

// pub fn get_loaded_program(address: &str)->String{
//     unimplemented!();
// }

// pub fn popup(address: String, popup_text:String) {
//     unimplemented!()
// }

// pub fn close_popup(address: &str){
//     unimplemented!()
// }

// pub fn add_to_log(address: String){

// }










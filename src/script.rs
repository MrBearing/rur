use std::fs::File;
use std::io;
use std::io::{/* BufReader ,*/ BufWriter};
use std::io::prelude::*;
use super::network::{write_to_tcp_stream,get_connected_tcp_stream};

fn read_file_string(file_path: &str) -> Result<String, io::Error> {
    let mut script_string = String::new();
    File::open(file_path)?.read_to_string(&mut script_string)?;
    Ok(script_string)
}


pub fn send(address :&str ,port: u32, script_file_path: &str){
    let script_string =  match read_file_string(script_file_path){
        Ok(s) => s,
        Err(e) => {
            eprintln!("Fail to read script file. cause:{}",e);
            return;
        },
    };

    let tcp_stream = match get_connected_tcp_stream(address, port){
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Fail to open TCP stream. cause: {}",e);
            return;
        }
    };
    let mut buf_writer = BufWriter::new(&tcp_stream);
    write_to_tcp_stream(&mut buf_writer, &script_string);
}


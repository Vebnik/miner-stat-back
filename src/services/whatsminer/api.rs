use serde_json::json;

use crate::services::whatsminer::types::{Client, Command, Summary};
use std::{io::{Read, Write}, net::TcpStream};


impl Client {
    pub fn new(addr: String, port: String, name: String, is_write: bool) -> Self {
        let url = format!("{}:{}", addr, port);
        let stream = TcpStream::connect(&url).expect("Error in crate tcp connect");

        let mut client = Self {addr, port, stream, name};

        if is_write { client.get_token() };

        client 
    }

    // private
    fn get_token(&mut self) -> () {
        let mut buffer = String::new();

        self.stream.write("{\"cmd\": \"get_token\"}".as_bytes()).unwrap();
        self.stream.read_to_string(&mut buffer).unwrap();

        let mut data = serde_json::from_str::<serde_json::Value>(&buffer)
            .expect("Error in parse json");

        let token_msg = data["Msg"].take();

        println!("{:?}", token_msg);
    }

    // pubs
    pub fn exec_command(&mut self, cmd_type: Command) -> serde_json::Value {
        let payload = json!({"cmd": cmd_type});
        let cmd = payload.to_string();
        let mut buffer = String::new();
        
        self.stream.write(cmd.as_bytes()).unwrap();
        self.stream.read_to_string(&mut buffer).unwrap();

        let raw_json: serde_json::Value = serde_json::from_str(&buffer)
            .expect("Error on parse summary");

        raw_json
    }

    pub fn summary(&mut self) -> Summary {
        let mut raw_data = self.exec_command(Command::Summary);

        serde_json::from_value::<Summary>(raw_data["SUMMARY"][0].take())
            .expect("Error on parsing summary")
    }
}
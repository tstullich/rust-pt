use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub enum Parser {
    OBJ(String),
}

impl Parser {
    pub fn parse(&self) {
        match &self {
            Parser::OBJ(path) => self.parse_obj(path),
        }
    }

    fn parse_obj(&self, path_name: &str) {
        let path = Path::new(path_name);
        let file = match File::open(path) {
            Err(_) => panic!("Couldn't open {:?}", path),
            Ok(file) => file,
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {
            match line.unwrap().get(..1) {
                Some("f") => println!("f"),
                Some("g") => println!("g"),
                Some("v") => println!("v"),
                _ => (),
            }
        }
    }
}

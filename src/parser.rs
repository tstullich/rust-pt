use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use HitableList;
use Material;
use Triangle;
use Vec3;

pub enum Parser {
    OBJ(String),
}

impl Parser {
    pub fn parse(&self) -> HitableList {
        match &self {
            Parser::OBJ(path) => self.parse_obj(path),
        }
    }

    fn parse_obj(&self, path_name: &str) -> HitableList {
        let path = Path::new(path_name);
        let file = match File::open(path) {
            Err(_) => panic!("Couldn't open {:?}", path),
            Ok(file) => file,
        };

        let reader = BufReader::new(file);
        let lines = reader.lines().map(|l| l.unwrap());
        let mut list = HitableList::new();
        let mut vector_table: Vec<Vec3> = Vec::new();
        for line in lines {
            let mut split = line.split_whitespace();
            let start_symbol = split.next();
            if start_symbol == Some("v") {
                let new_vec = Vec3::new(
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                    split.next().unwrap().parse::<f32>().unwrap(),
                );
                vector_table.push(new_vec);
            } else if start_symbol == Some("f") {
                let triangle = Triangle::new(
                    vector_table[split.next().unwrap().parse::<usize>().unwrap() - 1],
                    vector_table[split.next().unwrap().parse::<usize>().unwrap() - 1],
                    vector_table[split.next().unwrap().parse::<usize>().unwrap() - 1],
                    Material::Lambertian(Vec3::new(1.0, 0.0, 0.0)),
                );
                list.push(Box::new(triangle));
            }
        }
        list
    }
}

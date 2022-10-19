use serde::{Deserialize,Serialize};
use std::collections::HashMap;
use std::fmt::Error;
use std::fs::{self, File};
use std::hint;
use std::io::{BufReader, BufWriter};
#[derive(Debug,Deserialize,Serialize)]
struct Data {
    ver: isize,
    ksz: usize,
    value_sz: usize,
    key: String,
    value: String,
}

#[derive(Debug,Deserialize,Serialize)]
struct Hint {
    ver: isize,
    ksz: usize,
    value_sz: usize,
    key: String,
    value_pos: usize,
}

struct ValueIndex{
    file_id: usize,
    value_pos: usize
}

pub struct BitCask {
    index: HashMap<String,ValueIndex>,
    readers: HashMap<usize,BufReader<File>>,
    writer: BufWriter<File>,
}


impl BitCask {
    pub fn get_hint_ids(workDir: &str) -> std::io::Result<Vec<usize>>{
        let mut hint_ids = Vec::new();
        for _res in fs::read_dir(workDir)? {
            match _res {
                Ok(path) => {
                    let s= path.file_name().into_string().unwrap();
                    if s.ends_with(".hint") {
                        let id = usize::from_str_radix(s.strip_suffix(".hint").unwrap(), 10).unwrap();
                        hint_ids.push(id);
                    }
                }
                Err(_) => {}
            }
        }
        return Ok(hint_ids);
    }

    pub fn read_csv(path: &str) {
        let mut rdr = csv::Reader::from_path(path).unwrap();
        let Hint:Vec<Hint> = rdr.deserialize().map(|x|{x.unwrap()}).collect();
        for hint in Hint{
            println!("{}",hint.key);
        }
    }

    /*
    pub fn open(workDir: &str) -> std::io::Result<BitCask> {
        let paths =  fs::read_dir(workDir)?;
        let mut index = HashMap::new();
    }
    */
    
}
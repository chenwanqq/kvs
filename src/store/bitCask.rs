use serde::{Deserialize,Serialize};
use std::collections::HashMap;
use std::fmt::Error;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
#[derive(Debug,Deserialize,Serialize)]
struct Data {
    tstamp: i64,
    ksz: usize,
    value_sz: usize,
    key: String,
    value: String,
}

#[derive(Debug,Deserialize,Serialize)]
struct Hint {
    tstamp: i64,
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
    fn get_hint_ids(workDir: &str) -> std::io::Result<Vec<usize>>{
        let mut hint_ids = Vec::new();
        for _res in fs::read_dir(workDir) {
            
        }

    }
    pub fn open(workDir: &str) -> std::io::Result<BitCask> {
        let paths =  fs::read_dir(workDir)?;
        let mut index = HashMap::new();
    }
}
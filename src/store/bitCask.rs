use serde::{Deserialize,Serialize};
use std::collections::HashMap;
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
    fn get_hint_ids(workDir: &str) -> Result<Vec<usize>,&'static str>{
        let mut hint_ids = fs::read_dir(workDir).unwrap().map()
    }
    pub fn open(workDir: &str) -> Result<BitCask,&'static str> {
        let paths =  fs::read_dir(workDir).unwrap();
        let mut index = HashMap::new();
    }
}
use serde::{Deserialize,Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, BufWriter, Read, Seek, Write};
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
    readers: HashMap<usize,BufReader<R>>,
    writer: BufWriter<W>,
}


impl BitCask {
    fn open(workDir: &str) -> Result<BitCask,&'static str> {
        let paths =  fs::read_dir(workDir).unwrap();
        let mut index = HashMap::new();
    }
}
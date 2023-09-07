use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::BufRead;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard};
use once_cell::unsync::Lazy;
use crate::manual_destructor::ManualDestructor;
use crate::simple_hashset::{HASH_MAX, SimpleHashset};

type GlobalSet = Mutex<
    Lazy<
        ManualDestructor<
            SimpleHashset<
                HASH_MAX,String>
        >
    >
>;
static hash_set: GlobalSet = Mutex::new(Lazy::new(|| ManualDestructor::new(SimpleHashset::default())));

///Load the dictionary's lines into a hashmap
pub fn load(file_path: String) -> bool{
    let file = match std::fs::File::open(file_path) {
        Ok(v) => {v}
        Err(e) => {
            println!("Err {:?}", e);
            return false;
        }
    };
    let mut reader = std::io::BufReader::new(file);
    let mut map = hash_set.lock().unwrap();
    for line in reader.lines(){
        map.insert(line.unwrap().to_ascii_lowercase());
    }

    return true;
}

pub fn check(word: String) -> bool{
    hash_set.lock()
        .unwrap()
        .check(&word.to_ascii_lowercase())
}

pub fn unload() -> bool{
    hash_set.lock().unwrap().destroy();
    true
}

pub fn size() -> usize{
    hash_set.lock().unwrap().len()
}
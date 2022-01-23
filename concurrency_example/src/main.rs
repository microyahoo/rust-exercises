use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, RwLock};
use trace_caller::trace;

struct KvDb(Arc<Vec<RwLock<HashMap<String, Vec<u8>>>>>);

impl KvDb {
    #[trace]
    pub fn new(len: usize) -> Self {
        let mut dbs: Vec<RwLock<HashMap<String, Vec<u8>>>> = Vec::with_capacity(len);
        for _i in 0..len {
            dbs.push(RwLock::new(HashMap::new()))
        }
        Self(Arc::new(dbs))
    }

    pub fn insert(&self, k: &str, v: Vec<u8>) {
        let dbs = self.0.clone();
        let mut writer = dbs[(self.hash(k) % dbs.len()) as usize].write().unwrap();
        writer.insert(k.into(), v);
    }

    pub fn get(&self, k: &str) -> Vec<u8> {
        let dbs = self.0.clone();
        let reader = dbs[(self.hash(k) % dbs.len()) as usize].read().unwrap();
        reader.get(k).unwrap().to_owned()
    }

    fn hash(&self, k: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        k.to_owned().hash(&mut hasher);
        hasher.finish() as usize
    }
}

fn main() {
    println!("Hello, world!");

    let data = Mutex::new(0);
    let _d1 = data.lock();
    let _d2 = data.lock(); // deadlock now
}

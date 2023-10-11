use std::collections::HashMap;
use std::fmt::Debug;

pub fn print_hashmap<K: Debug, V: Debug>(hashmap: &HashMap<K, V>) {
    for (k, v) in hashmap.iter() {
        tracing::info!("k: {:?}, v: {:?}", k, v);
    }
}

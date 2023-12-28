use std::collections::HashMap;
use std::hash::Hash;

pub fn pickup_most_common_item<T: Ord + Hash + Clone>(items: &[T]) -> Option<T> {
    let mut item_to_count = HashMap::new();
    items.iter().for_each(|item| {
        *item_to_count.entry(item).or_insert(0) += 1;
    });

    let key = item_to_count
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _v)| k);

    if let Some(real) = key {
        return Some(real.clone().clone());
    }

    None
}

pub fn pickup_most_common_string(items: &[&String]) -> String {
    let mut item_to_count = HashMap::new();
    items.iter().for_each(|item| {
        if !item.is_empty() {
            *item_to_count.entry(item).or_insert(0) += 1;
        }
    });

    let key = item_to_count
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _v)| k);

    if let Some(real) = key {
        return real.to_owned().to_owned().to_owned();
    }

    return "".to_string();
}

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

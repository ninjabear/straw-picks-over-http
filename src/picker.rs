extern crate rand;

use rand::prelude::*;

pub fn pick_random<'a>(items: &Vec<&'a str>) -> &'a str {
    let idx = rand::thread_rng().gen_range(0, items.len());
    items[idx]
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    #[test]
    fn test_pick_random_appears_random() {
        let suggestion = vec!["one", "two"];
        let results: HashSet<&str> = (1..100).map(|_| super::pick_random(&suggestion)).collect();
        let mut result_options: Vec<&str> = results.into_iter().collect();
        result_options.sort();
        assert_eq!(vec!["one", "two"], result_options)
    }
}

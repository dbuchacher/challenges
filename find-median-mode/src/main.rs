/*
    Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;

fn median(list: &Vec<u16>) -> u16 {
    if list.len() % 2 == 0 {        // return median when list is even
        (list[list.len() / 2] + list[(list.len() / 2) - 1]) / 2
    } else {                        // return  median when list is odd
        list[(list.len() / 2)]
    }
}

fn mode(list: &Vec<u16>) -> u16 {
    let mut freq = HashMap::new();

    // populate hashmap with entry counts
    for i in list {
        let count = freq.entry(i).or_insert(0);
        *count += 1;
    }

    // find the most entry counts
    let mode: &u16 = freq
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(x, _)| x)
        .expect("some wrong with mode count");

    *mode
}

fn main() {
    // create list
    let mut list_one: Vec<u16> = vec![10, 3, 20, 3, 4];
    let mut list_two: Vec<u16> = vec![10, 3, 20, 3];

    // sort list lowest -> biggest
    list_one.sort();
    list_two.sort();

    // 3 3 4 10 20
    assert_eq!(median(&list_one), 4);
    assert_eq!(mode(&list_one), 3);

    // 3 3 10 20
    assert_eq!(median(&list_two), 6);
    assert_eq!(mode(&list_two), 3);
}

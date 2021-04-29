#![allow(dead_code)]
use std::collections::HashMap;

fn main() {}

//calculating the mean
fn mean(values: &Vec<i32>) -> f32 {
    let mut acc = 0.0;
    for val in values.iter() {
        acc += *val as f32;
    }
    let res = acc / values.len() as f32;
    res
}

#[test]
fn test_for_mean() {
    let values: Vec<i32> = vec![23, 43, 34, 43, 54, 50, 43, 52, 56];
    let the_mean = mean(&values);
    assert_eq!(the_mean, 44.22222);
}

//calculating the median
fn median(values: &mut Vec<i32>) -> i32 {
    let length = values.len();
    values.sort();

    let mid = length / 2;
    if length % 2 != 0 {
        values[mid]
    } else {
        mean(&vec![values[mid] - 1 + values[mid]]) as i32
    }
}

#[test]
fn median_test() {
    let mut values: Vec<i32> = vec![23, 43, 34, 43, 54, 50, 43, 52, 56];
    assert_eq!(median(&mut values), 43);
}

//calculating the mode
fn mode(values: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    for val in values {
        let count = occurrences.entry(val).or_insert(0);
        *count += 1;
    }

    let max_occurrences = occurrences.values().cloned().max().unwrap_or(0);
    let mut res = occurrences
        .into_iter()
        .filter(|&(_, v)| v == max_occurrences)
        .map(|(&k, _)| k)
        .collect::<Vec<i32>>();
    res.pop().unwrap()
}

#[test]
fn mode_test() {
    let values: Vec<i32> = vec![23, 43, 34, 43, 54, 50, 43, 52, 56];
    assert_eq!(mode(&values), 43);
}

//convert strings into pig latin
fn string_to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", chars.as_str(), first_char),
    }
}

#[test]
fn pig_latin_test() {
    assert_eq!(string_to_pig_latin("apple"), "apple-hay");
}

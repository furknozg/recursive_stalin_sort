// Import necessary crates
use crate::recursive_stalin_sort;

use rand::prelude::*;
use sorting_rs::merge_sort;
use std::time::Instant; // Import the Instant struct for measuring time

#[cfg(test)]
use crate::*;
use macros::ARRAY_SIZE;

#[test]
fn test_recursive_stalin_sort() {
    test_rs!(recursive_stalin_sort);
}

#[test]
fn test_selection_sort() {
    test_other!(selection_sort_rs);
}

#[test]
fn test_insertion_sort() {
    test_other!(insertion_sort_rs);
}

#[test]
fn test_merge_sort() {
    test_other!(merge_sort);
}

// Import necessary crates
use crate::recursive_stalin_sort;

use rand::prelude::*;
use std::time::Instant; // Import the Instant struct for measuring time

#[cfg(test)]
use crate::*;

#[test]
fn test_recursive_stalin_sort() {
    test_rs!(recursive_stalin_sort);
}

#[test]
fn test_merge_sort() {
    //test_rs!(merge_sort)
}

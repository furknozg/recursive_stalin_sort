use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Eq)]
struct Item<'a> {
    arr: &'a Vec<i32>,
    idx: usize,
}

impl<'a> PartialEq for Item<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.get_item() == other.get_item()
    }
}

impl<'a> PartialOrd for Item<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_item().partial_cmp(&other.get_item())
    }
}

impl<'a> Ord for Item<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_item().cmp(&other.get_item())
    }
}
impl<'a> Item<'a> {
    fn new(arr: &'a Vec<i32>, idx: usize) -> Self {
        Self { arr, idx }
    }

    fn get_item(&self) -> i32 {
        self.arr[self.idx]
    }
}

fn k_way_merge(arrays: &[Vec<i32>]) -> Vec<i32> {
    let mut sorted = vec![];

    let mut heap = BinaryHeap::with_capacity(arrays.len());
    for arr in arrays {
        let item = Item::new(arr, 0);
        heap.push(Reverse(item));
    }

    while !heap.is_empty() {
        let mut it = heap.pop().unwrap();
        sorted.push(it.0.get_item());
        it.0.idx += 1;
        if it.0.idx < it.0.arr.len() {
            heap.push(it)
        }
    }

    sorted
}

fn stalin_sort(nums: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut sorted = Vec::new();
    let mut eliminated = Vec::new();
    let mut last_sorted = i32::MIN;

    for num in nums {
        if *num >= last_sorted {
            sorted.push(*num);
            last_sorted = *num;
        } else {
            eliminated.push(*num);
        }
    }

    (sorted, eliminated)
}

pub fn recursive_stalin_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let mut partial_sorts = Vec::with_capacity(nums.len() / 2); // Estimated maximum size

    while !nums.is_empty() {
        let (sorted, eliminated) = stalin_sort(&mut nums);
        partial_sorts.push(sorted);
        nums = eliminated;
    }

    k_way_merge(&partial_sorts)
}

pub fn selection_sort_rs(arr: &mut [i32]) {
    let mut min: usize;
    let len = arr.len();
    for i in 1..len {
        min = i - 1;
        for j in i..len {
            unsafe {
                if arr.get_unchecked(min) > arr.get_unchecked(j) {
                    min = j;
                }
            }
        }
        arr.swap(i - 1, min);
    }
}

pub fn insertion_sort_rs(arr: &mut [i32]) {
    let len = arr.len();

    for i in 1..len {
        let current_value = arr[i];

        let mut prev: isize = (i - 1) as isize;
        while prev >= 0 && arr[prev as usize] >= current_value {
            arr[(prev + 1) as usize] = arr[prev as usize];
            prev -= 1;
        }
        arr[(prev + 1) as usize] = current_value;
    }
}

pub mod macros;
#[cfg(test)]
mod tests;

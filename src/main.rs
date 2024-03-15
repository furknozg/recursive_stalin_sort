fn stalin_sort(mut nums: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    // Edge case: if there are 0 or 1 elements, return the list as is
    if nums.len() <= 1 {
        return (nums, Vec::new());
    }

    let mut i = 1;
    let mut j = 0;

    // Initialize a vector to store eliminated elements
    let mut eliminated = Vec::new();

    // Iterate over the list
    while i < nums.len() {
        // If the current element is greater than or equal to the last sorted element,
        // Move it to its correct position in the sorted portion
        if nums[i] >= nums[j] {
            j += 1;
            nums.swap(i, j);
        } else {
            // If the current element is less than the last sorted element,
            // Remove it from the list and push it to the eliminated vector
            eliminated.push(nums[i]);
        }
        i += 1;
    }

    // Resize the vector to contain only the sorted portion
    nums.resize(j + 1, 0);

    (nums, eliminated)
}

fn recursive_stalin_sort(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut partial_sorts: Vec<Vec<i32>> = Vec::new();
    let (sorted, mut eliminated): (Vec<i32>, Vec<i32>) = stalin_sort(nums);
    partial_sorts.push(sorted);

    while !eliminated.is_empty() {
        let (sorted, new_eliminated) = stalin_sort(eliminated.clone());
        eliminated = new_eliminated;

        partial_sorts.push(sorted);
    }

    // start from the min index of the partially sorted arr, then apply sort as so

    return partial_sorts;
}

fn main() {
    let unsorted_nums = vec![10, 30, 20, 25, 30, 50, -1];
    println!("Unsorted: {:?}", unsorted_nums);

    let (sorted_nums, eliminated_nums) = stalin_sort(unsorted_nums.clone());
    println!("Sorted: {:?}", sorted_nums);
    println!("Eliminated: {:?}", eliminated_nums);

    let partial_sorts = recursive_stalin_sort(unsorted_nums);

    println!("Partially Sorted: {:?}", partial_sorts);
}

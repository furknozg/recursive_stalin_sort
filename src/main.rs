use stalinsort::recursive_stalin_sort;

fn main() {
    // Test cases
    let test_cases = vec![
        vec![3, 1, 5, 2, 4],
        vec![10, 30, 20, 25, 30, 50, -1],
        vec![5, 4, 3, 2, 1],
        vec![-5, -2, -7, -1, -3],
        vec![-10, -30, -20, -25, -30, -50, -1],
        vec![4, 3, 5, 2, 4],
        vec![10, 20, 30, 30, 40, 50, 40],
        vec![5, 2, 8, 10, 4, 6, 1, 3, 9, 7, 12, 15, 11, 14, 13],
    ];

    // Iterate over each test case and sort it
    for (index, test_case) in test_cases.iter().enumerate() {
        println!("Test case {}: {:?}", index + 1, test_case);

        // Sort the test case using stalin_sort function
        let partially_sorted = recursive_stalin_sort(test_case.clone());

        // Print the sorted and eliminated portions
        println!("Sorted: {:?}", partially_sorted);

        println!("{}", "#".repeat(30));
    }
}

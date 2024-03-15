#[macro_export]
macro_rules! test_rs {
    ( $fn:ident ) => {

        let mut rng = thread_rng();
        let arr: Vec<i32> = (0..100000).map(|_| rng.gen()).collect(); // Generate a random sequence of 1000 elements

        // Start the timer
        let start_time = Instant::now();
        let nums = $fn(arr.clone());

        // End the timer
        let elapsed_time = start_time.elapsed();
        println!("Recursive Stalin Sort: Time taken: {:?}", elapsed_time);

        let mut expected_sorted_nums: Vec<_> = arr.iter().cloned().collect();
        expected_sorted_nums.sort();
        // Perform assertions on the sorted result
        assert_eq!(nums, expected_sorted_nums);
    };
}

#[macro_export]
macro_rules! test_other {
    ($fn:ident) => {
        let mut rng = thread_rng();
        let arr: Vec<i32> = (0..10000).map(|_| rng.gen()).collect(); // Generate a random sequence of 1000 elements

        let mut arr_clone = arr.clone();
        $fn(&mut arr_clone);
        let nums = arr_clone;

        let mut expected_sorted_nums = arr.clone();
        expected_sorted_nums.sort();

        // Perform assertions on the sorted result
        assert_eq!(nums, expected_sorted_nums);
    };
}

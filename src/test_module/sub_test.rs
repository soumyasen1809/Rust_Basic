pub fn sub_testing_function() -> () {
    println!("Printing from a sub test");
    println!("Calling super here");
    super::print_test();
}

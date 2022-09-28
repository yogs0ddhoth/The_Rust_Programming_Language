use collections::{
    self, 
    hash_maps, 
    strings, 
    vectors,
    employee_interface
};
// collections are stored on the heap
fn main() {
    println!("VECTORS:\n");
    vectors::print_vectors();

    strings::print_strings();

    hash_maps::print_hash_maps();

    let list = [5,3,4,6,2,3,3,4,5,1];
    let (median, mode) = collections::median_n_mode(&list);
    println!(
        "list: {:?}\nmedian: {}, mode: {}",
        list, median, mode
    );
    println!(
        "{}", collections::translate_pig_latin("Hiya Chuck it's John")
    );
    employee_interface();
}


mod sort;

fn main() {
    call_sort();
}

fn call_sort() {
    // let mut inp_array = vec!["Ishan", "Shruti", "Krishna", "Ram", "Neelam", "Avval", "Avaan", "Ayaan"];
    let mut inp_array: Vec<u32> = vec![2,2,1,2,4,5,5,2,1,6,3];
    let length = inp_array.len();
    println!("Array is: {:?}", inp_array);
    inp_array = sort::quicksort(inp_array, 0, length);

    for val in inp_array {
        print!("{}\n", val)
    }
}

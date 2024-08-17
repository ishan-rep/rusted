mod sort;

fn main() {
    let mut inp_array = vec!["Ishan", "Shruti", "Krishna", "Ram", "Neelam", "Avval", "Avaan", "Ayaan"];
    let length = inp_array.len();
    println!("Array is: {:?}", inp_array);
    inp_array = sort::quicksort(inp_array, 0, length);

    for val in inp_array {
        print!("{}\n", val)
    }
}

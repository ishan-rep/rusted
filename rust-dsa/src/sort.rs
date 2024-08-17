/*
mergesort - recursive function. End is exclusive.
*/
pub fn mergesort<T: Ord + Clone>(inp_array: Vec<T>, start: usize, end: usize) -> Vec<T> {
    // 2 arrays - start to mid (exclusive) & mid to end (exclusive)
    if start >= end - 1 {
        return inp_array;
    }
    let mid = start + (end-start)/2;
    let arr_first_half = mergesort(inp_array[start..mid].to_vec(), start, mid);
    let arr_second_half = mergesort(inp_array[mid..end].to_vec(), 0, end-mid);

    return merge(arr_first_half, arr_second_half);
}

fn merge<T: Ord + Clone>(arr_first_half: Vec<T>, arr_second_half: Vec<T>) -> Vec<T> {

    let mut result_vector = Vec::new();

    // write the merging of 2 sorted arrays.
    let mut i = 0;
    let mut j= 0;
    let first_length = arr_first_half.len();
    let second_length = arr_second_half.len();

    while i < first_length && j < second_length {
        if arr_first_half[i] < arr_second_half[j] {
            result_vector.push(arr_first_half[i].clone());
            i += 1;
        }
        else {
            result_vector.push(arr_second_half[j].clone());
            j += 1;
        }
    }

    while i < first_length {
        result_vector.push(arr_first_half[i].clone());
        i += 1;
    }

    while j < second_length {
        result_vector.push(arr_second_half[j].clone());
        j += 1;
    }

    return result_vector;
}

// end is exclusive
pub fn quicksort<T: Ord + Clone + std::fmt::Debug>(mut inp_array: Vec<T>, start: usize, end: usize) -> Vec<T> {
    if start >= end - 1 {
        return inp_array;
    }


    let pivot = end-1;
    println!("pivot : {:?}", inp_array[pivot]);

    let mut i = start;
    let mut j = end-2;
    let mut temp;

    // Go till 
    while i <= j {
        if inp_array[i] < inp_array[pivot] {
            i += 1;
            continue;
        }
        else if inp_array[i] >= inp_array[pivot] {
            temp = inp_array[j].clone();
            inp_array[j] = inp_array[i].clone();
            inp_array[i] = temp;
            j -= 1;
            continue;
        }
    }
    println!("Array is: {:?}", inp_array);

    temp = inp_array[i].clone();
    inp_array[i] = inp_array[pivot].clone();
    inp_array[pivot] = temp;

    println!("Array is: {:?}", inp_array);

    inp_array = quicksort(inp_array, start, i);
    inp_array = quicksort(inp_array, i, end);
    return inp_array;
}

/*
mergesort - recursive function. End is exclusive.
*/
pub fn mergesort<T: Ord + Clone>(inp_array: Vec<T>, start: usize, end: usize) -> Vec<T> {
    // 2 arrays - start to mid (exclusive) & mid to end (exclusive)
    if start + 1 >= end {
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
    if start + 1 >= end {
        return inp_array;
    }
    let pivot = end-1;

    let mut i = start;
    let mut j = end-2;


    // Go till 
    while i <= j && j > 0 {
        if inp_array[i] <= inp_array[pivot] {
            i += 1;
            continue;
        }
        else if inp_array[i] > inp_array[pivot] {
            inp_array.swap(i, j);
            j -= 1;
            continue;
        }
    }
    inp_array.swap(i, pivot);
    inp_array = quicksort(inp_array, start, i);
    inp_array = quicksort(inp_array, i, end);
    return inp_array;
}

#[cfg(test)]
mod tests {
    use super::*; // This brings the quicksort and mergesort functions into scope.

    #[test]
    fn test_quicksort_empty() {
        let input: Vec<i32> = vec![];
        let result = quicksort(input.clone(), 0, input.len());
        assert_eq!(result, input);
    }

    #[test]
    fn test_quicksort_single_element() {
        let input = vec![1];
        let result = quicksort(input.clone(), 0, input.len());
        assert_eq!(result, input);
    }

    #[test]
    fn test_quicksort_multiple_elements() {
        let input = vec![3, 2, 5, 1, 4];
        let expected = vec![1, 2, 3, 4, 5];
        let result = quicksort(input.clone(), 0, input.len());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mergesort_empty() {
        let input: Vec<i32> = vec![];
        let result = mergesort(input.clone(), 0, input.len());
        assert_eq!(result, input);
    }

    #[test]
    fn test_mergesort_single_element() {
        let input = vec![1];
        let result = mergesort(input.clone(), 0, input.len());
        assert_eq!(result, input);
    }

    #[test]
    fn test_mergesort_multiple_elements() {
        let input = vec![3, 2, 5, 1, 4];
        let expected = vec![1, 2, 3, 4, 5];
        let result = mergesort(input.clone(), 0, input.len());
        assert_eq!(result, expected);
    }

    // Additional edge case tests can be added here...
}

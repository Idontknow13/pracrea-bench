mod divnconq;
mod heap;

//* QUADRATIC ALGORITHMS *//

pub fn bubble_sort<T: Ord + Clone>(values: &[T]) -> Vec<T> {
    let mut values = values.to_vec();
    if values.len() <= 1 {
        return values;
    }

    for _sorted in 0..values.len() {
        for current in 0..values.len() - 1 {
            if values[current] > values[current + 1] {
                values.swap(current, current + 1);
            }
        }
    }
    values
}

pub fn selection_sort<T: Ord + Clone>(values: &[T]) -> Vec<T> {
    let mut values = values.to_vec();
    for sorted in 0..values.len() {
        let mut min_idx = sorted;
        for other_idx in sorted + 1..values.len() {
            if values[other_idx] < values[min_idx] {
                min_idx = other_idx;
            }
        }
        if sorted != min_idx {
            // any changes = minimum in wrong place
            values.swap(sorted, min_idx);
        }
    }
    values
}

pub fn insertion_sort<T: Ord + Clone>(values: &[T]) -> Vec<T> {
    let mut values = values.to_vec();
    for curr_idx in 1..values.len() {
        for prev_idx in 0..curr_idx {
            if values[curr_idx] < values[prev_idx] {
                values.swap(curr_idx, prev_idx);
            }
        }
    }
    values
}

//* LINEAR-LOG ALGORITHMS *//

pub fn merge_sort<T: Ord + Clone>(values: &[T]) -> Vec<T> {
    if values.len() < 2 {
        return values.to_vec();
    }

    let middle = values.len() / 2;
    let left = merge_sort(&values[..middle]);
    let right = merge_sort(&values[middle..]);
    divnconq::merge(left, right)
}

pub fn quick_sort<T: Ord + Clone>(values: &[T]) -> Vec<T> {
    let mut values = values.to_vec();
    inplace_qsort(&mut values);
    values
}

fn inplace_qsort<T: Ord + Clone>(values: &mut [T]) {
    if values.len() < 2 {
        return;
    }

    let pivot_idx = divnconq::quick_partition(values);
    inplace_qsort(&mut values[..pivot_idx]);
    inplace_qsort(&mut values[pivot_idx + 1..]);
}

pub fn heap_sort<T: Ord + Clone>(values: &[T]) -> Vec<T> {
    let mut heap = heap::to_max_heap(values);
    let heapsize = heap.len();
    for sorted in 0..heapsize {
        // `heapsize - 1 - i` refers to last unsorted element
        heap.swap(0, heapsize - 1 - sorted);
        heap::max_heapify(&mut heap[..heapsize - 1 - sorted], 0);
    }
    heap
}

#[cfg(test)]
mod tests {
    use super::*;
    use fastrand;
    use std::iter::repeat_with;

    fn generate_vec(amt: usize) -> Vec<u8> {
        repeat_with(|| fastrand::u8(..)).take(amt).collect()
    }

    #[test]
    fn test_bubble_sort() {
        let mut values: Vec<_> = generate_vec(5);
        let sorted_with_algorithm = bubble_sort(&values);
        values.sort();
        assert_eq!(values, sorted_with_algorithm)
    }

    #[test]
    fn test_selection_sort() {
        let mut values: Vec<_> = generate_vec(5);
        let sorted_with_algorithm = selection_sort(&values);
        values.sort();
        assert_eq!(values, sorted_with_algorithm)
    }

    #[test]
    fn test_insertion_sort() {
        let mut values: Vec<_> = generate_vec(5);
        let sorted_with_algorithm = insertion_sort(&values);
        values.sort();
        assert_eq!(values, sorted_with_algorithm)
    }

    #[test]
    fn test_merge_sort() {
        let mut values: Vec<_> = generate_vec(5);
        let sorted_with_algorithm = merge_sort(&values);
        values.sort();
        assert_eq!(values, sorted_with_algorithm)
    }

    #[test]
    fn test_quick_sort() {
        let mut values: Vec<_> = generate_vec(5);
        let sorted_with_algorithm = quick_sort(&values);
        values.sort();
        assert_eq!(values, sorted_with_algorithm)
    }

    #[test]
    fn test_heap_sort() {
        let mut values: Vec<_> = generate_vec(5);
        let sorted_with_algorithm = heap_sort(&values);
        values.sort();
        assert_eq!(values, sorted_with_algorithm)
    }
}

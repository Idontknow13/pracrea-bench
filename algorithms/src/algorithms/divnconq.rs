pub fn merge<T>(left: Vec<T>, right: Vec<T>) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut merged_vec = vec![];
    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] < right[right_idx] {
            merged_vec.push(left[left_idx].clone());
            left_idx += 1;
        } else {
            merged_vec.push(right[right_idx].clone());
            right_idx += 1;
        }
    }

    if left_idx < left.len() {
        merged_vec.append(&mut left[left_idx..].to_vec())
    }
    if right_idx < right.len() {
        merged_vec.append(&mut right[right_idx..].to_vec())
    }

    merged_vec
}

pub fn quick_partition<T: Ord + Clone>(slice: &mut [T]) -> usize {
    let pivot = slice[slice.len() - 1].clone();
    let mut first_higher = 0;
    let mut curr = 0;

    while curr < slice.len() - 1 {
        if slice[curr] <= pivot {
            slice.swap(first_higher, curr);
            first_higher += 1;
        }
        curr += 1;
    }
    slice.swap(first_higher, slice.len() - 1);

    first_higher
}

#[cfg(test)]
mod divnconq_tests {
    use super::*;

    #[test]
    fn test_merge() {
        assert_eq!(vec![1, 2, 3], merge(vec![1], vec![2, 3]))
    }

    #[test]
    fn test_quick_partition() {
        let mut original_slice = vec![1, 3, 4, 2];
        let idx = quick_partition(&mut original_slice);
        assert_eq!([1, 2, 4, 3], original_slice.as_slice());
        assert_eq!(1, idx)
    }
}

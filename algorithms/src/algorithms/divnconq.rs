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

#[cfg(test)]
mod divnconq_tests {
    use super::*;

    #[test]
    fn test_merge() {
        assert_eq!(vec![1, 2, 3], merge(vec![1], vec![2, 3]))
    }
}

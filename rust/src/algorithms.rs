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
        assert_eq!(sorted_with_algorithm, values)
    }

    fn test_selection_sort() {
        todo!()
    }
}

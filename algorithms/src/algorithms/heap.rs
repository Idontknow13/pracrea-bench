pub fn to_max_heap<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    let mut heapable = arr.to_vec();
    for branch in (0..=heapable.len() / 2).rev() {
        max_heapify(&mut heapable, branch);
    }
    heapable
}

pub fn max_heapify<T: Ord>(heap: &mut [T], node_idx: usize) {
    let heapsize = heap.len();
    let (left_child, right_child) = if node_idx > 0 {
        (node_idx * 2, node_idx * 2 + 1)
    } else {
        (1, 2)
    };

    let mut largest = node_idx;
    // Find largest node
    if left_child < heapsize && heap[largest] < heap[left_child] {
        largest = left_child;
    }
    if right_child < heapsize && heap[largest] < heap[right_child] {
        largest = right_child;
    }
    // Swap + Base Condition
    if largest == node_idx {
        return;
    }
    heap.swap(node_idx, largest);
    // Recurse
    max_heapify(heap, largest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_heapify_start() {
        let mut heap = [3, 4, 5, 2, 1, 0];
        max_heapify(&mut heap, 0);
        assert_eq!([5, 4, 3, 2, 1, 0], heap);
    }

    #[test]
    fn test_max_heapify_middle() {
        let mut heap = [5, 4, 1, 2, 3, 0];
        max_heapify(&mut heap, 2);
        assert_eq!([5, 4, 3, 2, 1, 0], heap);
    }

    #[test]
    fn test_to_max_heap() {
        let heap = to_max_heap(&[2, 8, 5, 3, 9, 1]);
        assert_eq!([9, 8, 5, 3, 2, 1], heap.as_slice());
    }
}

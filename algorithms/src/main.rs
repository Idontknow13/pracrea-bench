mod algorithms;

use algorithms::{bubble_sort, heap_sort, insertion_sort, merge_sort, quick_sort, selection_sort};
use std::env::{args, Args};

// ! Change this with each new added algorithm
enum ImplementedAlgos {
    Bubble,
    Selection,
    Insertion,
    Merge,
    Quick,
    Heap,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (algoname, values) = get_important_args(args());
    let sort = map_algoname(&algoname)?.get_algorithm::<usize>();
    let _ = sort(&values);
    Ok(())
}

fn get_important_args(mut args: Args) -> (String, Vec<usize>) {
    let _binname = args.next(); // ignore binary name

    let algoname = args.next().expect("No provided algorithm name");
    let values = args
        .next()
        .map(|arg| {
            arg.split(',')
                .flat_map(|val| val.parse::<usize>())
                .collect::<Vec<_>>()
        })
        .expect("Provided invalid comma-separated values");

    (algoname, values)
}

fn map_algoname(algoname: &str) -> Result<ImplementedAlgos, &'static str> {
    let algoname = algoname.to_lowercase();
    match algoname.as_str() {
        "bubble-sort" | "bubble" => Ok(ImplementedAlgos::Bubble),
        "selection-sort" | "selection" => Ok(ImplementedAlgos::Selection),
        "insertion-sort" | "insertion" => Ok(ImplementedAlgos::Insertion),
        "merge-sort" | "merge" => Ok(ImplementedAlgos::Merge),
        "quick-sort" | "quick" => Ok(ImplementedAlgos::Quick),
        "heap-sort" | "heap" => Ok(ImplementedAlgos::Heap),
        _ => Err("Unimplemented algorithm chosen"),
    }
}

type SortingAlgo<T> = Box<dyn Fn(&[T]) -> Vec<T>>;
impl ImplementedAlgos {
    fn get_algorithm<T: Ord + Clone + 'static>(self) -> SortingAlgo<T> {
        Box::new(match self {
            Self::Bubble => bubble_sort,
            Self::Selection => selection_sort,
            Self::Insertion => insertion_sort,
            Self::Merge => merge_sort,
            Self::Quick => quick_sort,
            Self::Heap => heap_sort,
        })
    }
}

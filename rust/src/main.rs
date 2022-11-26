mod algorithms;

use algorithms::{bubble_sort, heap_sort};
use std::env::{args, Args};

// ! Change this with each new added algorithm
enum ImplementedAlgos {
    BubbleSort,
    HeapSort,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = args();
    let (algoname, values) = get_important_args(&mut args);
    let sort = map_algoname(&algoname)?.get_algorithm::<usize>();
    let _ = sort(&values);
    Ok(())
}

fn get_important_args(args: &mut Args) -> (String, Vec<usize>) {
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
        "bubble-sort" | "bubble" => Ok(ImplementedAlgos::BubbleSort),
        "heap-sort" | "heap" => Ok(ImplementedAlgos::HeapSort),
        _ => Err("Unimplemented algorithm chosen"),
    }
}

type SortingAlgo<T> = Box<dyn Fn(&[T]) -> Vec<T>>;
impl ImplementedAlgos {
    fn get_algorithm<T: Ord + Clone + 'static>(self) -> SortingAlgo<T> {
        Box::new(match self {
            Self::BubbleSort => bubble_sort,
            Self::HeapSort => heap_sort,
        })
    }
}

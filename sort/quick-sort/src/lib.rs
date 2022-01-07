#![feature(test)]

use std::fmt::Debug;

fn partition<T: PartialOrd + Debug>(xs: &mut [T]) -> usize {
    let last = xs.len() - 1;
    let mut i = 0;

    for j in 0..last {
        if xs[j] < xs[last] {
            xs.swap(i, j);
            i += 1;
        }
    }

    xs.swap(i, last);
    i + 1
}

// Lomuto partition scheme - the last item as pivot
pub fn quick_sort<T: PartialOrd + Debug>(xs: &mut [T]) {
    if xs.len() <= 1 {
        return;
    }

    let pivot = partition(xs);
    quick_sort(&mut xs[..pivot - 1]);
    quick_sort(&mut xs[pivot..]);
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use pretty_assertions::assert_eq;
    use rand::Rng;
    use test::Bencher;

    #[test]
    fn it_works_on_already_sorted_array() {
        let mut arr = [1, 2, 3];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_on_worse_case() {
        let mut arr = [3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_on_average_case() {
        let mut arr = [2, 3, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_on_array_with_even_numbers_of_elements() {
        let mut arr = [2, 7, 1, 3];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 7]);
    }

    #[test]
    fn it_works_with_complex_types() {
        let mut arr = ["c", "a", "b"];
        quick_sort(&mut arr);
        assert_eq!(arr, ["a", "b", "c"]);
    }

    #[bench]
    fn bench_best_case(b: &mut Bencher) {
        let mut arr: [i32; 1000] = (1..=1000)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong size iterator!");
        b.iter(|| quick_sort(&mut arr));
    }

    #[bench]
    fn bench_worse_case(b: &mut Bencher) {
        let mut arr: [i32; 1000] = (1..=1000)
            .rev()
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong size iterator!");
        b.iter(|| quick_sort(&mut arr));
    }

    #[bench]
    fn bench_random_case(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut arr: [i32; 1000] = (1..=1000)
            .map(|_x| rng.gen_range(1..=1000))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong size iterator!");
        b.iter(|| quick_sort(&mut arr));
    }
}

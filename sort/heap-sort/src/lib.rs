#![feature(test)]

use std::fmt::Debug;

enum Child {
    Left = 1,
    Right = 2,
}

fn parent(idx: usize) -> usize {
    (idx - 1) / 2
}

fn child(idx: usize, child_type: Child) -> usize {
    2 * idx + child_type as usize
}

fn sift_down<T: PartialOrd + Debug>(xs: &mut [T], start: usize) {
    println!("sift_down({:?}, {:?})", xs, start);
    let end = xs.len();
    let mut root = start;
    loop {
        let left = child(root, Child::Left);
        let right = child(root, Child::Right);
        let mut swap = root;
        if left < end && xs[swap] < xs[left] {
            swap = left;
        }
        if right < end && xs[swap] < xs[right] {
            swap = right;
        }
        if swap == root {
            return;
        }
        xs.swap(root, swap);
        root = swap;
    }
}

fn heapify<T: PartialOrd + Debug>(xs: &mut [T]) {
    let mut start = parent(xs.len() - 1);
    loop {
        sift_down(xs, start);
        if start == 0 {
            break;
        }
        start -= 1;
    }
}

pub fn heap_sort<T: PartialOrd + Debug>(xs: &mut [T]) {
    heapify(xs);
    let mut end = xs.len() - 1;
    while end > 0 {
        xs.swap(0, end);
        sift_down(&mut xs[..end], 0);
        end -= 1;
    }
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
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_on_worse_case() {
        let mut arr = [3, 2, 1];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_on_average_case() {
        let mut arr = [2, 3, 1];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_on_array_with_even_numbers_of_elements() {
        let mut arr = [2, 7, 1, 3];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 7]);
    }

    #[test]
    fn it_works_with_complex_types() {
        let mut arr = ["c", "a", "b"];
        heap_sort(&mut arr);
        assert_eq!(arr, ["a", "b", "c"]);
    }

    // Heapify
    #[test]
    fn it_can_create_a_heap_from_an_array() {
        let mut arr = [1, 2, 3];
        heapify(&mut arr);
        assert_eq!(arr, [3, 2, 1]);
    }

    #[test]
    fn it_can_create_a_heap_from_a_larger_array() {
        let mut arr = [22, 17, 19, 12, 15, 11, 7, 6, 9, 10, 5, 25];
        heapify(&mut arr);
        assert_eq!(arr, [25, 17, 22, 12, 15, 19, 7, 6, 9, 10, 5, 11]);
    }

    // Benchmarking
    #[bench]
    fn bench_best_case(b: &mut Bencher) {
        let mut arr: [i32; 1000] = (1..=1000)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong size iterator!");
        b.iter(|| heap_sort(&mut arr));
    }

    #[bench]
    fn bench_worse_case(b: &mut Bencher) {
        let mut arr: [i32; 1000] = (1..=1000)
            .rev()
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong size iterator!");
        b.iter(|| heap_sort(&mut arr));
    }

    #[bench]
    fn bench_random_case(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut arr: [i32; 1000] = (1..=1000)
            .map(|_x| rng.gen_range(1..=1000))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong size iterator!");
        b.iter(|| heap_sort(&mut arr));
    }
}

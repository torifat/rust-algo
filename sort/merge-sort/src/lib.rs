#![feature(test)]

use std::fmt::Debug;
use std::mem::swap;

fn merge<'a, T: PartialOrd>(left: &mut [T], right: &mut [T], buffer: &mut [T]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut buffer_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            swap(&mut buffer[buffer_index], &mut left[left_index]);
            left_index += 1;
        } else {
            swap(&mut buffer[buffer_index], &mut right[right_index]);
            right_index += 1;
        }
        buffer_index += 1;
    }

    while left_index < left.len() {
        swap(&mut buffer[buffer_index], &mut left[left_index]);
        left_index += 1;
        buffer_index += 1;
    }

    while right_index < right.len() {
        swap(&mut buffer[buffer_index], &mut right[right_index]);
        right_index += 1;
        buffer_index += 1;
    }
}

fn merge_sort_helper<T: PartialOrd + Debug>(xs: &mut [T], buffer: &mut [T]) {
    let len = xs.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let (left, right) = xs.split_at_mut(mid);
    let (left_buffer, right_buffer) = buffer.split_at_mut(mid);

    merge_sort_helper(left, left_buffer);
    merge_sort_helper(right, right_buffer);
    merge(left, right, buffer);
    xs.swap_with_slice(buffer);
}

pub fn merge_sort<T: PartialOrd + Default + Debug>(xs: &mut [T]) {
    let mut buffer = (0..xs.len()).map(|_| T::default()).collect::<Vec<T>>();
    merge_sort_helper(xs, &mut buffer)
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
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_on_worse_case() {
        let mut arr = [3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_on_average_case() {
        let mut arr = [2, 3, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_on_array_with_even_numbers_of_elements() {
        let mut arr = [2, 7, 1, 3];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 7]);
    }

    #[test]
    fn it_works_with_complex_types() {
        let mut arr = ["c", "a", "b"];
        merge_sort(&mut arr);
        assert_eq!(arr, ["a", "b", "c"]);
    }

    // merge
    #[test]
    fn it_can_merge_two_empty_arrays() {
        let mut left = [0; 0];
        let mut right = [0; 0];
        let mut buffer = [0; 0];
        merge(&mut left, &mut right, &mut buffer);
        assert_eq!(buffer, []);
    }

    #[test]
    fn it_can_merge_one_empty_array_with_one_non_empty_array() {
        let mut left = [0; 0];
        let mut right = [1; 1];
        let mut buffer = [0; 1];
        merge(&mut left, &mut right, &mut buffer);
        assert_eq!(buffer, [1]);
    }

    #[test]
    fn it_can_merge_two_non_empty_arrays() {
        let mut left = [1, 2, 3];
        let mut right = [4, 5, 6];
        let mut buffer = [0; 6];
        merge(&mut left, &mut right, &mut buffer);
        assert_eq!(buffer, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn it_can_merge_two_arrays_of_different_lengths() {
        let mut left = [1, 2, 3];
        let mut right = [4, 5];
        let mut buffer = [0; 5];
        merge(&mut left, &mut right, &mut buffer);
        assert_eq!(buffer, [1, 2, 3, 4, 5]);
    }

    // Benchmarking
    #[bench]
    fn bench_best_case(b: &mut Bencher) {
        let mut arr: [i32; 1000] = (1..=1000)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong size iterator!");
        b.iter(|| merge_sort(&mut arr));
    }

    #[bench]
    fn bench_worse_case(b: &mut Bencher) {
        let mut arr: [i32; 1000] = (1..=1000)
            .rev()
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong size iterator!");
        b.iter(|| merge_sort(&mut arr));
    }

    #[bench]
    fn bench_random_case(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut arr: [i32; 1000] = (1..=1000)
            .map(|_x| rng.gen_range(1..=1000))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong size iterator!");
        b.iter(|| merge_sort(&mut arr));
    }
}

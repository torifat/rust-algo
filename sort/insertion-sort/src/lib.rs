#![feature(test)]

pub fn insertion_sort<T: PartialOrd>(xs: &mut [T]) {
    let len = xs.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && xs[j - 1] > xs[j] {
            xs.swap(j - 1, j);
            j -= 1;
        }
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
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_on_worse_case() {
        let mut arr = [3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_on_average_case() {
        let mut arr = [2, 3, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_with_complex_types() {
        let mut arr = ["c", "a", "b"];
        insertion_sort(&mut arr);
        assert_eq!(arr, ["a", "b", "c"]);
    }

    #[bench]
    fn bench_best_case(b: &mut Bencher) {
        let mut arr: [u32; 1000] = (1..=1000)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong size iterator!");
        b.iter(|| insertion_sort(&mut arr));
    }

    #[bench]
    fn bench_worse_case(b: &mut Bencher) {
        let mut arr: [u32; 1000] = (1..=1000)
            .rev()
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong size iterator!");
        b.iter(|| insertion_sort(&mut arr));
    }

    #[bench]
    fn bench_random_case(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut arr: [u32; 1000] = (1..=1000)
            .map(|_x| rng.gen_range(1..=1000))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Wrong size iterator!");
        b.iter(|| insertion_sort(&mut arr));
    }
}

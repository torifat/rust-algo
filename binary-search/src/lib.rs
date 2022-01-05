pub fn binary_search<T: PartialOrd>(xs: &[T], x: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = xs.len();
    while low < high {
        // https://ai.googleblog.com/2006/06/extra-extra-read-all-about-it-nearly.html
        // Credits to Donald Knuth for this mid point formula
        let mid = (low & high) + ((low ^ high) >> 1);
        let cur = &xs[mid];
        if *cur > *x {
            high = mid;
        } else if *cur < *x {
            low = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_returns_none_when_array_is_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, &1), None);
    }

    #[test]
    fn it_returns_none_when_not_found_in_odd_array() {
        let arr = [0];
        assert_eq!(binary_search(&arr, &1), None);
    }

    #[test]
    fn it_returns_none_when_not_found_in_even_array() {
        let arr = [0, 1];
        assert_eq!(binary_search(&arr, &2), None);
    }

    #[test]
    fn it_returns_index_when_first_item_found() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, &1), Some(0));
    }

    #[test]
    fn it_returns_index_when_middle_item_found() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, &3), Some(2));
    }

    #[test]
    fn it_returns_index_when_last_item_found() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, &5), Some(4));
    }

    #[test]
    fn it_returns_none_when_item_not_found() {
        let arr = [2, 4, 6, 8, 10];
        assert_eq!(binary_search(&arr, &9), None);
    }

    #[test]
    fn it_works_with_floating_number() {
        let arr = [1.0, 1.5, 2.0];
        assert_eq!(binary_search(&arr, &1.5), Some(1));
    }

    #[test]
    fn it_works_with_string() {
        let arr = ["apple", "banana", "cherry"];
        assert_eq!(binary_search(&arr, &"banana"), Some(1));
    }

    #[test]
    fn it_finds_all_numbers() {
        let arr = [-9, -2, 0, 3, 4, 6, 9, 19, 25, 33, 39, 52, 88, 91, 99];
        for (idx, x) in arr.iter().enumerate() {
            assert_eq!(binary_search(&arr, &x), Some(idx));
        }
    }
}

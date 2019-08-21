pub fn generic_sort<T>(items: &mut Vec<T>)
where
    T: std::cmp::Ord,
{
    items.sort();
}

pub struct Point<U, V> {
    x: U,
    y: V,
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_fn_generic_sort() {
        let mut nums: Vec<usize> = [2, 3, 1, 59013, 132, 1328].to_vec();
        super::generic_sort(&mut nums);

        assert_eq!(nums[0], 1);
        assert_eq!(nums[1], 2);
        assert_eq!(nums[2], 3);
        assert_eq!(nums[3], 132);
        assert_eq!(nums[4], 1328);
        assert_eq!(nums[5], 59013);
    }

    #[test]
    fn test_generic_struct() {
        let p = super::Point { x: 1.0, y: -12 };
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, -12);
    }

    #[test]
    #[ignore]
    // cargo test -- --ignore
    fn ignored_test() {
        assert!(true);
    }
}

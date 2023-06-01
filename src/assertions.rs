
#[macro_export]
macro_rules! assert_eq_set {
    ($left:expr, $right:expr) => {
        {
            let only_in_left = $left.difference(&$right).cloned().collect::<HashSet<(u8, u8)>>();
            let only_in_right = $right.difference(&$left).cloned().collect::<HashSet<(u8, u8)>>();

            if only_in_left.len() > 0 && only_in_right.len() > 0 {
                assert_eq!(only_in_left, only_in_right, "There were unique elements in both sets")
            }
            assert_eq!(only_in_left, HashSet::new(), "There were unique elements in the left set");
            assert_eq!(HashSet::new(), only_in_right, "There were unique elements in the right set");
        }
    }
}
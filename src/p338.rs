struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..=n).into_iter().map(|i| i.count_ones() as i32).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}

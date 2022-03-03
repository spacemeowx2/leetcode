struct Solution;

impl Solution {
    /// You are given a sorted unique integer array nums.
    ///
    /// Return the smallest sorted list of ranges that cover all the numbers in the array exactly.
    /// That is, each element of nums is covered by exactly one of the ranges, and there is
    /// no integer x such that x is in one of the ranges but not in nums.
    ///
    /// Each range [a,b] in the list should be output as:
    ///
    ///     "a->b" if a != b
    ///     "a" if a == b
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        use std::iter::once;

        fn print(start: i32, end: i32) -> String {
            if start == end {
                format!("{}", start)
            } else {
                format!("{}->{}", start, end)
            }
        }
        let mut state = (nums[0], nums[0]);
        nums.into_iter()
            .map(Option::Some)
            .chain(once(None))
            .scan(&mut state, |(start, end), x| {
                Some(match x {
                    Some(x) => {
                        if x == *end {
                            *end += 1;
                            None
                        } else {
                            let s = print(*start, *end - 1);
                            *start = x;
                            *end = x + 1;
                            Some(s)
                        }
                    }
                    None => Some(print(*start, *end - 1)),
                })
            })
            .flat_map(|x| x)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        );
        assert!(Solution::summary_ranges(vec![]).is_empty())
    }
}

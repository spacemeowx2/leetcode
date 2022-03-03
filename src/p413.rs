struct Solution;

const STOP: i32 = -10000;

fn sub_sum(n: i32) -> i32 {
    (1..=(n - 2)).sum()
}

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        // state: (diff, last number, count)
        let state = (STOP, STOP, 0);
        nums.into_iter()
            .chain(std::iter::once(STOP))
            .scan(state, |(diff, last, count), i| {
                let mut ret = None;
                if i == STOP || i - *last != *diff {
                    // the sequence is broken, so we need to reset the state
                    ret = Some(*count);
                    *diff = i - *last;
                    *count = 2;
                } else {
                    // i - *last == *diff
                    *count += 1;
                }
                *last = i;
                Some(ret)
            })
            .flat_map(|x| x)
            .fold(0, |acc, x| acc + sub_sum(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sub_sum(0), 0);
        assert_eq!(sub_sum(3), 1);
        assert_eq!(sub_sum(4), 3);
        assert_eq!(sub_sum(5), 6);
    }

    #[test]
    fn test_0() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![1, 2, 3, 8, 9, 10]),
            2
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4, 5, 6]),
            10
        );
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3]), 1);
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
    }
}

struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut out = vec![];
        for row in matrix.iter() {
            let (id, min) =
                row.iter()
                    .enumerate()
                    .fold((usize::MAX, i32::MAX), |(min_id, min), (id, &i)| {
                        if i < min {
                            (id, i)
                        } else {
                            (min_id, min)
                        }
                    });
            if !matrix.iter().any(|r| r[id] > min) {
                out.push(min);
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1380() {
        assert_eq!(Solution::lucky_numbers(vec![]), vec![]);
        assert_eq!(
            Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
            vec![15]
        );
        assert_eq!(
            Solution::lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ]),
            vec![12]
        );
        assert_eq!(
            Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2],]),
            vec![7]
        );
    }
}

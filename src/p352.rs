use std::cmp::Ordering;

struct SummaryRanges {
    ranges: Vec<(i32, i32)>,
}

impl SummaryRanges {
    fn new() -> Self {
        SummaryRanges { ranges: vec![] }
    }

    fn add_num(&mut self, val: i32) {
        let r = self.ranges.binary_search_by(|&(l, r)| {
            if val < l {
                Ordering::Greater
            } else if val > r {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        match r {
            Ok(i) => {
                let (l, r) = &mut self.ranges[i];
                *l = (*l).min(val);
                *r = (*r).max(val);
            }
            Err(i) => {
                self.ranges.insert(i, (val, val));
            }
        };
        // merge
        let mut i = 0;
        while i < self.ranges.len() - 1 {
            let (l1, r1) = self.ranges[i];
            let (l2, r2) = self.ranges[i + 1];
            if r1 + 1 >= l2 {
                self.ranges.remove(i + 1);
                self.ranges[i] = (l1.min(l2), r1.max(r2));
            } else {
                i += 1;
            }
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.ranges.iter().map(|&(l, r)| vec![l, r]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut obj = SummaryRanges::new();
        obj.add_num(1);
        assert_eq!(obj.get_intervals(), vec![vec![1, 1]]);
        obj.add_num(3);
        assert_eq!(obj.get_intervals(), vec![vec![1, 1], vec![3, 3]]);
        obj.add_num(7);
        assert_eq!(
            obj.get_intervals(),
            vec![vec![1, 1], vec![3, 3], vec![7, 7]]
        );
        obj.add_num(2);
        assert_eq!(obj.get_intervals(), vec![vec![1, 3], vec![7, 7]]);
        obj.add_num(6);
        assert_eq!(obj.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
    }
}

// Problem 3479
impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        /*
         * segment tree + binary search
         */

        use std::cmp::max;

        fn build(nodes: &mut [i32], idx: usize, baskets: &[i32], lo: usize, hi: usize) {
            if lo == hi {
                nodes[idx] = baskets[lo];
                return;
            }

            let mi = (lo + hi) / 2;

            build(nodes, 2 * idx + 1, baskets, lo, mi);
            build(nodes, 2 * idx + 2, baskets, mi + 1, hi);

            nodes[idx] = max(nodes[2 * idx + 1], nodes[2 * idx + 2]);
        }

        fn consume(nodes: &mut [i32], idx: usize, lo: usize, hi: usize, quantity: i32) -> usize {
            if nodes[idx] < quantity {
                return usize::MAX;
            }

            if lo == hi {
                nodes[idx] = -1;
                return lo;
            }

            let mi = (lo + hi) / 2;
            let pos = if nodes[2 * idx + 1] >= quantity {
                consume(nodes, 2 * idx + 1, lo, mi, quantity)
            } else {
                consume(nodes, 2 * idx + 2, mi + 1, hi, quantity)
            };
            nodes[idx] = max(nodes[2 * idx + 1], nodes[2 * idx + 2]);
            pos
        }

        let n = fruits.len();
        assert!(n >= 1 && baskets.len() == n);

        let mut count = 0;
        let mut nodes = vec![0; n * 4];

        build(&mut nodes, 0, &baskets, 0, n - 1);
        // println!("{:?}", nodes);

        for quantity in fruits.iter().copied() {
            let pos = consume(&mut nodes, 0, 0, n - 1, quantity);
            if pos == usize::MAX {
                count += 1;
            }
        }

        count
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Problem 2968
impl Solution {
    pub fn max_frequency_score(mut nums: Vec<i32>, k: i64) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        nums.sort();

        let nums = nums;

        let prefix_sums = [0i32]
            .iter()
            .chain(nums.iter())
            .scan(0i64, |acc, x| {
                *acc += *x as i64;
                Some(*acc)
            })
            .collect::<Vec<i64>>();

        // Facts:
        // 1. Given a window of sorted numbers, partition it into two partitions, left and right.
        //      The cost = SUM( T - a_i ) + SUM( a_j  - T ), where T is a target number,
        //      a_i are numbers in left partition, a_j are numbers in right partition, and
        //      max(a_i) <= T <= min(a_j),
        //
        // 2. The best choice of target number depends on the partition sizes:
        //      a. the target number is cancelled out if equal size
        //      b. max(left partition) if len(left partition) > len(right partition)
        //      c. min(min partition) otherwise.
        //      In case (b) and (c), the cost calculation can be simplified by excluding the target
        //      from either partition
        //
        // 3. The cost is minimized if abs(len(left) - len(right)) <= 1.
        //      This means the target number is around the middle of the window.

        // Algorithm
        // 1. Start with a window of size 1 at the beginning of the sorted sequence
        // 2. If cost <= k, extend the window size to right by 1, go step 4.
        // 3. If cost > k, move window right by 1
        // 4. If window out of bounds, stop!
        // 5. Go step 2

        // Index:   i   i+1 i+2 i+3 i+3
        //          win     mid     stop
        //
        // window [i, i+3) is partition into left=[win, mid) and right=[mid, stop)
        // SUM(left) = a_i + .. + a_(mid - 1) = PRESUM(mid) - PRESUM(i)
        // SUM(right) = a_mid + ... + a_stop = PRESUM(stop) - PRESUM(mid)

        let mut max_score = 1i32;
        let mut window = 0usize;
        let mut stop = 1;

        // println!("{:?}", nums);

        while stop <= nums.len() {
            // If they are too close, target may be out of window
            if stop - window == 1 {
                stop += 1;
                continue;
            }

            let mid = (window + stop) / 2;
            // If the window size is even, the target number does not matter.
            // If odd, the target number can be assigned to either left or right partition.
            let cost = if (stop - window) % 2 == 0 {
                // window size is even
                prefix_sums[stop] + prefix_sums[window] - prefix_sums[mid] * 2
            } else {
                // window size is odd
                prefix_sums[stop] + prefix_sums[window] - prefix_sums[mid] - prefix_sums[mid + 1]
            };

            // println!("window=[{}, {}], cost={}", window, stop, cost);

            if cost > k {
                // window too big, but its ok to keep it unchaged
                // becauase we are finding the maximum window size
                // move the window to right
                window += 1;
            } else {
                max_score = max_score.max((stop - window) as i32);
                stop += 1;
            }
        }

        max_score
    }
}

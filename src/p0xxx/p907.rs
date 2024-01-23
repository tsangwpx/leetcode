mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 907
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        assert!(arr.len() >= 1);

        const MOD: i64 = 10i64.pow(9) + 7;

        let mut sum = 0;

        // strictly increasing stack of indexes
        let mut stack = Vec::with_capacity(arr.len());

        for idx in 0..arr.len() {
            let value = arr[idx];

            // maintain the stack invariant by popping larger or equal value
            while let Some(&mid) = stack.last() {
                if arr[mid] < value {
                    break;
                }

                stack.pop().unwrap();
                let minimum = arr[mid];
                let start = stack.last().copied().map(|s| s + 1).unwrap_or(0);
                let stop = idx;
                // When we poppped an index, mid,
                // arr[mid] is the minimum of subarrays of arr[i..j]
                // where start <= i <= mid and mid < j <= stop
                // start = stack[-1] + 1 or 0 if stack is empty
                // stop = idx, which is exclusive
                // so the number of possible i = mid - start
                // and the number of possible j = stop - mid
                // and the number of subarrays are i * j
                let count = (mid + 1 - start) * (stop - mid);
                sum += minimum as i64 * count as i64;
                sum %= MOD;
            }

            stack.push(idx);
        }

        // Similar argument, but stop is constant and equal to arr.len()

        while let Some(mid) = stack.pop() {
            let minimum = arr[mid];
            let start = stack.last().copied().map(|s| s + 1).unwrap_or(0);
            let count = (mid + 1 - start) * (arr.len() - mid);
            sum += minimum as i64 * count as i64;
            sum %= MOD;
        }

        sum as i32
    }

    pub fn sum_subarray_mins2(arr: Vec<i32>) -> i32 {
        // Time complexity: O(N**2) because
        // 1. We traverse each item O(N)
        // 2. Maintain a stack of O(N) and traverse it everytime
        assert!(arr.len() >= 1);

        const MOD: i64 = 10i64.pow(9) + 7;

        let mut sum = i64::from(arr[0]) % MOD;

        // strictly increasing stack of indexes
        let mut stack = Vec::with_capacity(arr.len());
        stack.push(0usize);

        for idx in 1..arr.len() {
            let value = arr[idx];

            // maintain the stack invariant by popping larger or equal value
            // we have min([start, stack[i]]) = stack[i],
            // where stack[i - 1] < start <= stack[i] for i > 0
            while let Some(&top) = stack.last() {
                if arr[top] >= value {
                    // or we may calculate the sum here when an index is popped?
                    // also after the main loop, to pop all indexes to empty the stack
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(idx);

            let mut start = 0;
            for i in 0..stack.len() {
                let stop = stack[i] + 1;
                let count = (stop - start) as i64;
                let minimum = arr[stack[i]] as i64;
                sum += minimum * count;
                start = stop;
                // println!("{}: add {} {} {}", idx, value, start, stop - start);
            }

            sum %= MOD;

            // println!("{}: sum={}", idx, sum);
            // println!("{}: stack={:?}", idx, stack);
        }

        sum as i32
    }
}

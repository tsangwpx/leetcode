fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

// Problem 238
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        assert!(nums.len() >= 2);

        let mut out = vec![1i32; nums.len()];

        // Build prefix product except the last item and store it in output vec
        let mut product = 1;
        for i in 0..(nums.len() - 1) {
            product *= nums[i];
            out[i] = product;
        }

        // The last item get its left friend as the final product
        out[nums.len() - 1] = out[nums.len() - 2];

        // Going backward to compute the suffix product
        // and combine with the prefix result stored in output to get the final output vec
        let mut product = 1;
        for i in (1..nums.len() - 1).rev() {
            product *= nums[i + 1];
            out[i] = out[i - 1] * product;
        }
        product *= nums[1];
        out[0] = product;

        out
    }

    pub fn product_except_self2(mut nums: Vec<i32>) -> Vec<i32> {
        assert!(nums.len() >= 2);

        // build suffix product in output from stop to start
        let mut out = vec![0i32; nums.len()];

        //
        out[nums.len() - 1] = nums[nums.len() - 1];

        for i in (0..nums.len()).rev().skip(1) {
            out[i] = nums[i] * out[i + 1];
        }

        // build prefix product in input from start to stop
        for i in 1..nums.len() {
            nums[i] = nums[i] * nums[i - 1];
        }

        // now build the product without self
        out[0] = out[1];

        for i in 1..(nums.len() - 1) {
            out[i] = out[i + 1] * nums[i - 1];
        }

        out[nums.len() - 1] = nums[nums.len() - 2];

        out
    }
}

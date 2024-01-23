// mod done;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Problem 46
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Self::sjt_algo(nums)
        Self::heap_algo(nums)
    }

    // Heap's algorithm
    fn heap_algo(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn generate(res: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, k: usize) {
            if k == 1 {
                res.push(nums.clone());
            } else {
                generate(res, nums, k - 1);

                for i in 0..(k - 1) {
                    if k % 2 == 0 {
                        nums.swap(i, k - 1);
                    } else {
                        nums.swap(0, k - 1);
                    }

                    generate(res, nums, k - 1);
                }
            }
        }

        let total = (2..=nums.len()).fold(1, |a, b| a * b);
        let mut res = Vec::with_capacity(total);
        let k = nums.len();
        generate(&mut res, &mut nums, k);
        res
    }

    // Steinhaus–Johnson–Trotter algorithm
    fn sjt_algo(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 1 {
            return vec![nums];
        }

        let total = (2..=nums.len()).fold(1, |a, b| a * b);
        let mut res = Vec::with_capacity(total);
        let mut count = 0;

        loop {
            for i in (0..nums.len()).skip(1) {
                res.push(nums.clone());
                count += 1;

                if count >= total {
                    return res;
                }

                nums.swap(i, i - 1);
            }
        }
    }
}

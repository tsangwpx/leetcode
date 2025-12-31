use std::cmp::Ordering;

/*
nums: [5, 7, 1, 8]
1 -100000 5 7 0
2 5 7 1 0
3 7 1 8 0
nums: [-1, 4, 2, 3]
1 -100000 -1 4 0
2 -1 4 2 0
3 -1 2 3 1
nums: [3, 4, 2, 3]
1 -100000 3 4 0
2 3 4 2 0
3 4 2 3 0
nums: [4, 2, 3]
1 -100000 4 2 0
2 -100000 2 3 1
nums: [4, 2, 1]
1 -100000 4 2 0
2 -100000 2 1 1




 */
impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return true;
        }

        let mut modified = 0;
        let mut prev = -100000;

        // println!("nums: {:?}", nums);

        let mut pos = 0;
        while pos < nums.len() - 1 {
            let number = nums[pos];
            pos += 1;
            let next = nums[pos];
            // println!("{} {} {} {} {}", pos, prev, number, next, modified);

            use std::cmp::Ordering;

            match number.cmp(&prev) {
                Ordering::Equal => {
                    prev = number;
                }
                Ordering::Less => {
                    //                  ------
                    //  ------          ^next1
                    //  ^prev           ------
                    //          ------  ^next2, impossible
                    //          ^curr   ------
                    //                  ^next3, impossible
                    if prev > next {
                        return false;
                    }

                    // set current to prev, so prev is unchanged
                    modified += 1;
                    if modified >= 2 {
                        break;
                    }
                }
                Ordering::Greater => {
                    //                          fut1
                    //                  next1
                    //          curr            fut2
                    //                  next2,
                    //  prev                    fut3
                    //                  next3, dep on future
                    //                          fut4
                    if number > next {
                        if prev <= next {
                            // Set current to prev, so prev is unchanged
                            modified += 1;
                            if modified >= 2 {
                                break;
                            }
                        } else {
                            // prev > next
                            // change current does not fix the problem, so leave it
                            prev = number;
                        }
                    } else {
                        prev = number;
                    }
                }
            }
        }

        // check the last number, modify it if needed
        if nums[nums.len() - 1] < prev {
            modified += 1;
        }

        modified < 2 // total edits must be less than 2
    }
}

fn main() {}

from leetcode_prelude import *


# Problem 2563
class Solution:
    def countFairPairs(self, nums: List[int], lower: int, upper: int) -> int:
        """
        1. 0 <= i < j < n imply that a pair is formed in combination sense
        2. we may sort the array in this sense
        3. two pointer approach to count sums less than or equal to a value
        4. the difference between two counts is the answer
        """
        if len(nums) < 2:
            return 0

        nums.sort()

        def count_le(value: int) -> int:
            count = 0
            left = 0
            right = len(nums) - 1

            while left < right:
                while left < right and nums[left] + nums[right] > value:
                    right -= 1

                count += right - left
                left += 1

            return count

        return count_le(upper) - count_le(lower - 1)

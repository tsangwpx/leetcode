from leetcode_prelude import *


# Problem 1509
class Solution:
    def minDifference(self, nums: List[int]) -> int:
        # 4 possible ways
        # 1. remove 3 largest items
        # 2. remove 2 largest items + 1 smallest item
        # 3. remove 1 largest items + 2 smallest item
        # 4. remove 3 smallest items

        if len(nums) <= 4:
            return 0

        nums.sort()

        return min(
            nums[-4] - nums[0],  # case 1
            nums[-3] - nums[1],  # case 2
            nums[-2] - nums[2],  # case 3
            nums[-1] - nums[3],  # case 3
        )

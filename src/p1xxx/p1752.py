from leetcode_prelude import *


# Problem 1752
class Solution:
    def check(self, nums: List[int]) -> bool:
        # slow but work!
        for idx in range(1, len(nums)):
            if nums[idx - 1] > nums[idx]:
                break
        else:
            idx = len(nums)

        return sorted(nums) == nums[idx:] + nums[0:idx]

from leetcode_prelude import *


# Problem 2779
class Solution:
    def maximumBeauty(self, nums: List[int], k: int) -> int:
        nums.sort()

        two_k = k * 2
        removed = 0

        for number in nums:
            lower = number - two_k
            if nums[removed] < lower:
                removed += 1

        return len(nums) - removed

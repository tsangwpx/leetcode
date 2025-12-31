from leetcode_prelude import *


# Problem 2270
class Solution:
    def waysToSplitArray(self, nums: List[int]) -> int:
        total = sum(nums)

        left = 0
        right = total
        count = 0
        for number in nums[:-1]:
            left += number
            right -= number
            if left >= right:
                count += 1
        return count

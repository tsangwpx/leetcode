from leetcode_prelude import *


# Problem 1608
class Solution:
    def specialArray(self, nums: List[int]) -> int:
        from bisect import bisect_left

        nums.sort()

        size = len(nums)

        # because nums is small, do a linear loop here
        # binary search is also possible

        for target in range(size + 1):
            idx = bisect_left(nums, target)
            count = size - idx

            if target == count:
                return target

        return -1

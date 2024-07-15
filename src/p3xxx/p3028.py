from leetcode_prelude import *


# Problem 3028
class Solution:
    def returnToBoundaryCount(self, nums: List[int]) -> int:
        pos = 0
        count = 0

        for step in nums:
            pos += step

            if pos == 0:
                count += 1

        return count

from leetcode_prelude import *


# Problem 2966
class Solution:
    def divideArray(self, nums: List[int], k: int) -> List[List[int]]:
        nums.sort()

        res = []

        for i in range(0, len(nums), 3):
            row = nums[i : i + 3]
            if row[0] + k < row[2]:
                return []

            res.append(row)

        return res

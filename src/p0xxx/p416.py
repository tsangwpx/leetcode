from typing import List

class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)

        if total % 2 != 0:
            return False
        target = total // 2

        dp = 0

        for number in nums:
            dp = dp | ((dp | 1) << number)
            # print(number, bin(dp))

        flag = 1 << target
        return (dp & flag) == flag

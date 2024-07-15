from leetcode_prelude import *


# Problem 3158
class Solution:
    def duplicateNumbersXOR(self, nums: List[int]) -> int:
        from collections import Counter

        counter = Counter(nums)
        res = 0

        for value, num in counter.items():
            if num == 2:
                res = res ^ value

        return res

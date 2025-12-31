from leetcode_prelude import *


# Problem 1636
class Solution:
    def frequencySort(self, nums: List[int]) -> List[int]:
        from collections import Counter

        counter = Counter(nums)

        pairs = sorted(counter.items(), key=lambda s: (s[1], -s[0]))
        res = []

        for item, count in pairs:
            res.extend([item] * count)

        return res

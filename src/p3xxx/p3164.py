from leetcode_prelude import *


# Problem 3164
class Solution:
    def numberOfPairs(self, nums1: List[int], nums2: List[int], k: int) -> int:
        from collections import Counter
        from math import isqrt

        counter = [0] * (max(nums1) + 1)

        for base, freq in Counter(nums2).items():
            base *= k

            for factor in range(base, len(counter), base):
                counter[factor] += freq

        return sum([counter[number] * freq for number, freq in Counter(nums1).items()])

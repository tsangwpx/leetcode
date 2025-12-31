from leetcode_prelude import *


# Problem 2425
class Solution:
    def xorAllNums(self, nums1: List[int], nums2: List[int]) -> int:
        x = 0

        if len(nums1) % 2 == 1:
            for number in nums2:
                x ^= number

        if len(nums2) % 2 == 1:
            for number in nums1:
                x ^= number

        return x

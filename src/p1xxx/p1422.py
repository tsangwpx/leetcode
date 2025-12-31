from leetcode_prelude import *


# Problem 1422
class Solution:
    def maxScore(self, s: str) -> int:
        total_zeros = s.count("0")

        left_zeros = 0
        right_ones = len(s) - total_zeros

        maximum = 0

        # dont visit the last item.
        # otherwise the right string will be empty in the last iteration.
        for ch in s[:-1]:
            if ch == "0":
                left_zeros += 1
            elif ch == "1":
                right_ones -= 1

            maximum = max(maximum, left_zeros + right_ones)

        return maximum

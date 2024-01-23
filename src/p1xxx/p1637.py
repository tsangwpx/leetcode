from leetcode_prelude import *


# Problem 1637
class Solution:
    def maxWidthOfVerticalArea(self, points: List[List[int]]) -> int:
        xs = [s for s, _ in points]
        xs.sort()
        return max(b - a for a, b in zip(xs[0:-1], xs[1:]))

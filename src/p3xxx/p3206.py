from leetcode_prelude import *


# Problem 3206
class Solution:
    def numberOfAlternatingGroups(self, colors: List[int]) -> int:
        count = 0

        for i in range(len(colors) - 2):
            if colors[i] != colors[i + 1] and colors[i + 1] != colors[i + 2]:
                count += 1

        if colors[-2] != colors[-1] and colors[-1] != colors[0]:
            count += 1

        if colors[-1] != colors[0] and colors[0] != colors[1]:
            count += 1

        return count

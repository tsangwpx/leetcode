from leetcode_prelude import *


# Problem 330
class Solution:
    def minPatches(self, nums: List[int], n: int) -> int:
        nums.reverse()

        patches = 0

        upper = 1  # exclusive

        while upper <= n:
            if nums and nums[-1] <= upper:
                shift = nums.pop()
            else:
                shift = upper
                patches += 1

            upper = upper + shift
            # print(len(nums), shift, patches)

        return patches

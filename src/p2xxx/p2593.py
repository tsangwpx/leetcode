from leetcode_prelude import *


# Problem 2593
class Solution:
    def findScore(self, nums: List[int]) -> int:
        order = [(s, i) for i, s in enumerate(nums)]
        order.sort()

        # allocate extra slot for access by -1 and len(nums)
        marked = [False] * (len(nums) + 1)
        score = 0

        for item, idx in order:
            if marked[idx]:
                continue

            score += item

            marked[idx] = True
            marked[idx - 1] = True
            marked[idx + 1] = True

        return score

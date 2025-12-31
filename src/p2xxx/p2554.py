from leetcode_prelude import *


# Problem 2554
class Solution:
    def maxCount(self, banned: List[int], n: int, maxSum: int) -> int:
        NUM_MAX: int = 10000
        skipped = [False] * (NUM_MAX + 1)

        for num in banned:
            skipped[num] = True

        count = 0
        remaining = maxSum
        for num in range(1, n + 1):
            if skipped[num]:
                continue
            if num > remaining:
                break
            count += 1
            remaining -= num

        return count

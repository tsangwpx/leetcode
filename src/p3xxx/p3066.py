from leetcode_prelude import *


# Problem 3066
class Solution:
    def minOperations(self, nums: List[int], k: int) -> int:
        from heapq import heapify, heappop, heappush

        heapify(nums)

        count = 0

        while len(nums) >= 2:
            x = heappop(nums)

            if x >= k:
                break

            count += 1
            y = heappop(nums)
            z = min(x, y) * 2 + max(x, y)
            heappush(nums, z)

        return count

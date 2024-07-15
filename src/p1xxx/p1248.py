from leetcode_prelude import *


# Problem 1248
class Solution:
    def numberOfSubarrays(self, nums: List[int], k: int) -> int:
        from collections import deque

        indices = deque()

        left = 0
        count = 0

        for idx, number in enumerate(nums):
            if number % 2 == 1:
                indices.append(idx)

            if len(indices) > k:
                left = indices.popleft() + 1

            if len(indices) == k:
                count += indices[0] + 1 - left

            # print(idx, number, indices, left, count)

        return count

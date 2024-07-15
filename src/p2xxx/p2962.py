from leetcode_prelude import *


# Problem 2962
class Solution:
    def countSubarrays(self, nums: List[int], k: int) -> int:
        from collections import deque

        maximum = max(nums)

        dq = deque()
        count = 0
        start = 0

        for idx, number in enumerate(nums):
            if number == maximum:
                dq.append(idx)

            if len(dq) >= k:
                start = dq.popleft() + 1

            # print(idx, start, count)
            count += idx + 1 - start

        size = len(nums)
        res = (1 + size) * size // 2 - count

        return res

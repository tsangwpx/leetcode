from leetcode_prelude import *


# Problem 3264
class Solution:
    def getFinalState(
        self,
        nums: List[int],
        k: int,
        multiplier: int,
    ) -> List[int]:
        from heapq import heapify, heappop, heappush, heapreplace

        pq = [(number, idx) for idx, number in enumerate(nums)]
        heapify(pq)

        for _ in range(k):
            number, idx = pq[0]
            heapreplace(pq, (number * multiplier, idx))

        for number, idx in pq:
            nums[idx] = number

        return nums

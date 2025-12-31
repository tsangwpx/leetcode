from leetcode_prelude import *


# Problem 2558
class Solution:
    def pickGifts(self, gifts: List[int], k: int) -> int:
        from math import isqrt
        from heapq import heapify, heappop, heappush

        # convert to negative number in min-heap
        pq = [-s for s in gifts]
        heapify(pq)

        for _ in range(k):
            if not pq:
                break

            count = -heappop(pq)
            left = isqrt(count)
            if left:
                heappush(pq, -left)

        return -sum(pq)

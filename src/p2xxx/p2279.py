from leetcode_prelude import *


# Problem 2279
class Solution:
    def maximumBags(
        self,
        capacity: List[int],
        rocks: List[int],
        additionalRocks: int,
    ) -> int:
        from heapq import heapify, heappop, heappush

        pq = []

        for size, amount in zip(capacity, rocks):
            pq.append(size - amount)

        heapify(pq)

        full = 0

        while pq:
            amount = heappop(pq)
            additionalRocks -= amount

            if additionalRocks >= 0:
                full += 1
            else:
                break

        return full

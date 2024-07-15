from leetcode_prelude import *


# Problem 786
class Solution:
    def kthSmallestPrimeFraction(self, arr: List[int], k: int) -> List[int]:
        """Implementation without floating numbers"""
        from functools import cmp_to_key
        from heapq import heapify, heappop, heappush, heapreplace

        def cmp(t1: tuple[int, int], t2: tuple[int, int]) -> int:
            a1, b1 = t1
            a2, b2 = t2

            return arr[a1] * arr[b2] - arr[a2] * arr[b1]

        key_fn = cmp_to_key(cmp)

        pq = [(key_fn((0, s)), (0, s)) for s in range(1, len(arr))]
        heapify(pq)

        for _ in range(k - 1):
            _, (a, b) = pq[0]
            a += 1
            item = (a, b)
            heapreplace(pq, (key_fn(item), item))

        _, (a, b) = pq[0]
        return [arr[a], arr[b]]

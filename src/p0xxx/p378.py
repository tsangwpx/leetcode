from leetcode_prelude import *


# Problem 378
class Solution:
    def kthSmallest(self, matrix: List[List[int]], k: int) -> int:
        from heapq import heapify, heappop, heappush

        n = len(matrix)

        pq = [(matrix[s][0], s, 0) for s in range(n)]
        heapify(pq)

        for _ in range(k - 1):
            _, row, col = heappop(pq)
            col += 1
            if col < n:
                heappush(pq, (matrix[row][col], row, col))

        res, _, _ = pq[0]
        return res

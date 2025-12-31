from leetcode_prelude import *


# Problem 2940
class Solution:
    def leftmostBuildingQueries(
        self,
        heights: List[int],
        queries: List[List[int]],
    ) -> List[int]:
        """
        1. Three cases:
            - If they are already in the same building, done.
            - If the right person is strictly higher, the right person just need to stay, done.
            - If the left person is equal or higher to the other, see below
        2. We can only process a query if we would have passed the right person.
            Scan from left to right, and answer unfulfilled query on the left.
            Use min-heap to store unanswered queries, using the right person height.
            Add new queries in i-th position after the scan have just passed the i-ith index.
        """
        from heapq import heappop, heappush

        res = [-1] * len(queries)
        queued = [[] for _ in range(len(heights))]
        pq = []

        for qi, (ai, bi) in enumerate(queries):
            # who is on the left or right does not matter.
            if ai > bi:
                ai, bi = bi, ai

            ah = heights[ai]
            bh = heights[bi]

            if ai == bi or ah < bh:
                res[qi] = bi
            else:
                queued[bi].append((max(ah, bh), qi))

        for i, h in enumerate(heights):
            while pq and pq[0][0] < h:
                _, qi = heappop(pq)
                res[qi] = i

            for hmax, qi in queued[i]:
                heappush(pq, (hmax, qi))

        return res

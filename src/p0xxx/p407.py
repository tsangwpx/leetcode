# Problem 407
class Solution:
    def trapRainWater(self, heightMap: List[List[int]]) -> int:
        from heapq import heappop, heappush, heapify
        from collections import defaultdict

        m = len(heightMap)
        n = len(heightMap[0])

        pq = []
        queued = defaultdict(bool)

        for i in range(m):
            pq.append((heightMap[i][0], i, 0))
            pq.append((heightMap[i][n - 1], i, n - 1))
            queued[i, 0] = True
            queued[i, n - 1] = True

        for j in range(n):
            pq.append((heightMap[0][j], 0, j))
            pq.append((heightMap[m - 1][j], m - 1, j))
            queued[0, j] = True
            queued[m - 1, j] = True

        filled = 0

        # pq is a queue of cell leaking water
        heapify(pq)

        while pq:
            h, i, j = heappop(pq)
            for p, q in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]:
                if not 0 <= p < m or not 0 <= q < n or queued[p, q]:
                    continue
                # find a new cell that has not been queued
                # if this cell is lower than mine
                # fill it up to my height
                filled += max(0, h - heightMap[p][q])

                # then queue it
                heappush(pq, (max(heightMap[p][q], h), p, q))
                queued[p, q] = True

        return filled

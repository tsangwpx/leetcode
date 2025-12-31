from leetcode_prelude import *


# Problem 1765
class Solution:
    def highestPeak(self, isWater: List[List[int]]) -> List[List[int]]:
        from collections import deque

        m = len(isWater)
        n = len(isWater[0])

        # use dict for convenience; list for space and speed
        queue = deque()

        # the result matrix
        heights = [[-1] * n for _ in range(m)]

        # Add all water cell to the queue
        # and set their heights to be zero
        for i, row in enumerate(isWater):
            for j, is_water in enumerate(row):
                if not is_water:
                    continue

                queue.append((i, j))
                heights[i][j] = 0

        # the heights whose cell queued are non-decreasing
        # we are safe to assign h + 1 to unvisited cell

        while queue:
            i, j = queue.popleft()
            h = heights[i][j]

            for p, q in ((i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)):
                if not (0 <= p < m and 0 <= q < n):
                    continue

                # skip cell set previously
                if heights[p][q] >= 0:
                    continue

                heights[p][q] = h + 1
                queue.append((p, q))

        return heights

from leetcode_prelude import *


# Problem 2290
class Solution:
    def minimumObstacles(
        self,
        grid: List[List[int]],
    ) -> int:
        from collections import deque

        m = len(grid)
        n = len(grid[0])

        MAX = 10**7

        dist = [[m + n] * n for _ in range(m)]
        dist[0][0] = 0

        dq = deque()
        dq.appendleft((0, 0, 0))

        while dq:
            i, j, cost = dq.popleft()

            for i2, j2 in ((i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)):
                if not 0 <= i2 < m or not 0 <= j2 < n:
                    continue

                new_cost = cost + 1 if grid[i2][j2] else cost
                if new_cost >= dist[i2][j2]:
                    continue

                if grid[i2][j2]:
                    dist[i2][j2] = new_cost
                    dq.append((i2, j2, new_cost))
                else:
                    dist[i2][j2] = new_cost
                    dq.appendleft((i2, j2, new_cost))

        return dist[m - 1][n - 1]

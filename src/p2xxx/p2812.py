from leetcode_prelude import *


# Problem 2812
class Solution:
    def maximumSafenessFactor(self, grid: List[List[int]]) -> int:
        from collections import deque
        from heapq import heappop, heappush

        if grid[0][0] or grid[-1][-1]:
            # Special case
            return 0

        n = len(grid)

        # Cell safety values default to 9999
        safeties = [[9999] * n for _ in range(n)]

        # Find all thieves and propagate their influence with BFS
        # Update queue, item = (safety value, row, col)
        pending = deque()

        for i in range(n):
            for j in range(n):
                if grid[i][j]:
                    pending.append((0, i, j))

        while pending:
            val1, row, col = pending.popleft()
            val0 = safeties[row][col]

            if val1 >= val0:
                # No change
                continue

            safeties[row][col] = val1
            val1 += 1

            for i, j in (
                (row - 1, col),
                (row + 1, col),
                (row, col - 1),
                (row, col + 1),
            ):
                if i < 0 or i >= n or j < 0 or j >= n:
                    continue

                # We may check change here to avoid muting the queue
                pending.append((val1, i, j))

        # search with heap which try the safest path first.
        pq = [(-safeties[0][0], 0, 0)]

        seen = [[False] * n for _ in range(n)]
        seen[0][0] = True

        while pq:
            safeness, row, col = heappop(pq)
            safeness = -safeness

            for i, j in (
                (row - 1, col),
                (row + 1, col),
                (row, col - 1),
                (row, col + 1),
            ):
                if i < 0 or i >= n or j < 0 or j >= n:
                    continue

                if seen[i][j]:
                    continue

                seen[i][j] = True

                # Since the next cell is reached from the safest path in the heap,
                # the path safeness along the next cell is
                # minimum of both path safeness and cell safety.
                next_safeness = min(safeness, safeties[i][j])

                # return if reach the target
                if i + 1 == n and j + 1 == n:
                    return next_safeness

                heappush(pq, (-next_safeness, i, j))

        raise AssertionError("unreachable")

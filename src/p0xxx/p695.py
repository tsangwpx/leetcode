import collections
from typing import List, Tuple


class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])

        pq = []
        islands: List[List[Tuple[int, int]]] = []
        mappings: List[List[int]] = [[-1] * n for _ in range(m)]

        def propagate_left(i: int, j: int, target: int, value: int):
            for j in range(j):
                if mappings[i][j] == target:
                    mappings[i][j] = value

        def propagate_top(i: int, j: int, target: int, value: int):
            if i >= 1:
                i -= 1

                for j in range(j, n):
                    if mappings[i][j] == target:
                        mappings[i][j] = value

        #
        # for i, row in enumerate(grid):
        #     print(f'row {i:2d}', ''.join('x' if s else '_' for s in row))

        for i in range(m):
            for j in range(n):
                if not grid[i][j]:
                    continue

                top_island = mappings[i - 1][j] if i >= 1 else -1
                left_island = mappings[i][j - 1] if j >= 1 else -1

                if top_island >= 0:
                    my_island = top_island
                    if left_island >= 0 and top_island != left_island:
                        islands[top_island].extend(islands[left_island])
                        propagate_left(i, j, left_island, top_island)
                        propagate_top(i, j, left_island, top_island)

                elif left_island >= 0:
                    my_island = left_island
                else:
                    my_island = len(islands)
                    islands.append([])

                mappings[i][j] = my_island
                islands[my_island].append((i, j))

            # print(f"map {i:2d}", ' '.join(f'{s:2d}' if s >= 0 else '__' for s in mappings[i]))

        return max((len(s) for s in islands), default=0)

    def maxAreaOfIsland2(self, grid: List[List[int]]) -> int:
        maximum = 0
        m = len(grid)
        n = len(grid[0])

        def find_area(i, j):
            area = 0

            q = collections.deque()
            q.append((i, j))

            while q:
                i, j = q.popleft()
                if 0 <= i < m and 0 <= j < n and grid[i][j]:
                    grid[i][j] = 0
                    area += 1
                    q.extend(((i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)))

            return area

        for i in range(m):
            for j in range(n):
                maximum = max(maximum, find_area(i, j))

        return maximum

from leetcode_prelude import *


# Problem 827
class Solution:
    def largestIsland(self, grid: List[List[int]]) -> int:
        n = len(grid)
        # union find

        def get_neighbors(i, j):
            for x, y in (
                (i - 1, j),
                (i, j - 1),
                (i, j + 1),
                (i + 1, j),
            ):
                if 0 <= x < n and 0 <= y < n:
                    yield x, y

        def find_root(i, j) -> tuple[int, int]:
            r = parent[i][j]
            if r == (i, j):
                return r
            ri, rj = r
            r = find_root(ri, rj)
            parent[i][j] = r
            return r

        def merge(i1, j1, i2, j2) -> None:
            r1 = find_root(i1, j1)
            r2 = find_root(i2, j2)
            if r1 == r2:
                return

            r3 = min(r1, r2)
            i1, j1 = r1
            i2, j2 = r2
            i3, j3 = r3

            parent[i1][j1] = r3
            parent[i2][j2] = r3
            members[i3][j3] = members[i1][j1] + members[i2][j2]

        parent: list[list[tuple[int, int]]] = []
        members: list[list[int]] = []

        for i in range(n):
            parent.append([(i, j) for j in range(n)])
            members.append([grid[i][j] for j in range(n)])

        for i in range(n):
            for j in range(n):
                if grid[i][j] == 0:
                    continue
                for neighbor in get_neighbors(i, j):
                    ni, nj = neighbor
                    if grid[ni][nj]:
                        merge(i, j, ni, nj)

        max_island = max(max(row) for row in members)

        for i in range(n):
            for j in range(n):
                if grid[i][j] == 1:
                    continue
                area = 1
                roots = set(find_root(si, sj) for si, sj in get_neighbors(i, j))
                for ri, rj in roots:
                    if grid[ri][rj]:
                        area += members[ri][rj]

                max_island = max(max_island, area)

        return max_island

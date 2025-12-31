from leetcode_prelude import *


# Problem 1267
class Solution:
    def countServers(self, grid: list[list[int]]) -> int:
        m = len(grid)
        n = len(grid[0])

        parents: dict[tuple[int, int], tuple[int, int]] = {}
        sizes: dict[tuple[int, int], int] = {}
        row_parents: list[tuple[int, int] | None] = [None] * m
        col_parents: list[tuple[int, int] | None] = [None] * n

        def find(key: tuple[int, int]) -> tuple[int, int]:
            root = parents[key]
            if key != root:
                root = find(root)
                parents[key] = root

            return root

        def union(key: tuple[int, int], key2: tuple[int, int]) -> tuple[int, int]:
            # print("union", key, key2)
            root = find(key)
            root2 = find(key2)

            if root == root2:
                return root

            root3 = min(root, root2)
            parents[root] = root3
            parents[root2] = root3
            sizes[root3] = sizes[root] + sizes[root2]
            return root3

        for i, row in enumerate(grid):
            for j, has_server in enumerate(grid[i]):
                if not has_server:
                    continue

                key = (i, j)
                parents[key] = key
                sizes[key] = 1

                if row_parents[i] is None:
                    row_parents[i] = key
                else:
                    union(row_parents[i], key)

                if col_parents[j] is None:
                    col_parents[j] = key
                else:
                    union(col_parents[j], key)

        connected = 0
        for key in parents.keys():
            root = find(key)
            if key == root and sizes[key] >= 2:
                connected += sizes[root]

        return connected

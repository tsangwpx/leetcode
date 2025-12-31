from leetcode_prelude import *


# Problem 2392
class Solution:
    def buildMatrix(
        self,
        k: int,
        rowConditions: List[List[int]],
        colConditions: List[List[int]],
    ) -> List[List[int]]:

        # transform to 0-based numbering
        rowConditions = [(a - 1, b - 1) for a, b in rowConditions]
        colConditions = [(a - 1, b - 1) for a, b in colConditions]

        def sort_numbers(edges: list[tuple[int, int]]) -> list[int]:
            # topo sort

            # rev_depends[A] = [B, ...]
            # B depends on A
            rev_deps = [[] for _ in range(k)]
            degrees = [0] * k

            for src, dst in edges:
                rev_deps[src].append(dst)
                degrees[dst] += 1

            pending = [i for i, d in enumerate(degrees) if d == 0]
            res = []

            while pending:
                top = pending.pop()
                res.append(top)

                for next_ in rev_deps[top]:
                    degrees[next_] -= 1
                    if degrees[next_] == 0:
                        pending.append(next_)

            return res

        rows = sort_numbers(rowConditions)

        if len(rows) != k:
            return []

        cols = sort_numbers(colConditions)
        if len(cols) != k:
            return []

        def get_indices(numbers) -> list[int]:
            indices = [0] * k
            for idx, num in enumerate(numbers):
                indices[num] = idx
            return indices

        # Is there any way to swap the key-value pair of a dict-like array
        # because this is some kind of permutation.
        row_indices = get_indices(rows)
        col_indices = get_indices(cols)

        matrix = [[0] * k for _ in range(k)]

        for num, (i, j) in enumerate(zip(row_indices, col_indices)):
            matrix[i][j] = num + 1

        return matrix

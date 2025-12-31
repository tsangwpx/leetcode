from leetcode_prelude import *


# Problem 1334
class Solution:
    def findTheCity(
        self,
        n: int,
        edges: List[List[int]],
        distanceThreshold: int,
    ) -> int:
        # Floyd Warshall algorithm, O(N**3)

        MAX = 10**5

        dists = [[MAX] * n for _ in range(n)]

        for i in range(n):
            dists[i][i] = 0

        for a, b, w in edges:
            dists[a][b] = min(dists[a][b], w)
            dists[b][a] = min(dists[b][a], w)

        for k in range(n):
            for i in range(n):
                for j in range(n):
                    if dists[i][k] + dists[k][j] < dists[i][j]:
                        dists[i][j] = dists[i][k] + dists[k][j]

        res = 0
        res_count = n

        for i in range(n):
            count = 0
            for j in range(n):
                if dists[i][j] <= distanceThreshold:
                    count += 1

            # print(i, count)

            if count <= res_count:
                res = i
                res_count = count

        return res

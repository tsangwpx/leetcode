from leetcode_prelude import *


# Problem 3243
class Solution:
    def shortestDistanceAfterQueries(
        self,
        n: int,
        queries: List[List[int]],
    ) -> List[int]:
        from heapq import heappush, heappop

        dists = list(range(n))
        edges = [[i + 1] for i in range(n - 1)]
        edges.append([])

        pq = []
        res = []

        for src, dst in queries:
            edges[src].append(dst)

            src_dist = dists[src]
            dst_dist = dists[dst]
            new_dist = src_dist + 1

            if new_dist < dst_dist:
                dists[dst] = new_dist

                heappush(pq, (new_dist, dst))

                while pq:
                    cost, node = heappop(pq)

                    if cost > dists[node]:
                        continue

                    cost += 1

                    for next_node in edges[node]:
                        if dists[next_node] > cost:
                            dists[next_node] = cost
                            heappush(pq, (cost, next_node))

            res.append(dists[-1])

        return res

from leetcode_prelude import *


# Problem 2045
class Solution:
    def secondMinimum(
        self,
        n: int,
        edges: List[List[int]],
        time: int,
        change: int,
    ) -> int:
        from heapq import heappush, heappop

        period = change * 2
        neighbors = [[] for _ in range(n)]

        for a, b in edges:
            a -= 1
            b -= 1

            neighbors[a].append(b)
            neighbors[b].append(a)

        MAX = 10**8

        # store the first minimum and second minimum
        dists = [(MAX, MAX) for _ in range(n)]
        dists[0] = (0, MAX)
        pq = [(0, 0)]

        while pq:
            cost, node = heappop(pq)

            if cost > dists[node][-1]:
                # old record
                continue

            # print(node, cost, dists[node])
            # print(dists)

            # compute the arrival time of the next node
            q, r = divmod(cost, period)
            if r < change:
                new_cost = cost
            else:
                new_cost = period * (q + 1)
            new_cost += time

            for neighbor in neighbors[node]:
                first, second = dists[neighbor]

                # update the arrival time if better and different
                if new_cost >= second or new_cost == first:
                    continue

                if new_cost < first:
                    dists[neighbor] = (new_cost, first)
                else:
                    dists[neighbor] = (first, new_cost)

                # schedule
                heappush(pq, (new_cost, neighbor))

        return dists[n - 1][1]

    def secondMinimum2(
        self,
        n: int,
        edges: List[List[int]],
        time: int,
        change: int,
    ) -> int:
        from heapq import heappush, heappop

        period = change * 2

        neighbors = [[] for _ in range(n)]
        for a, b in edges:
            a -= 1
            b -= 1

            neighbors[a].append(b)
            neighbors[b].append(a)

        def dijkstra(
            pq: list[tuple[int, int]],
            best: list[int],
            dists: list[int],
        ):
            """
            Perform dijkstra's algorithm
            until the priority queue is consumed

            dists is the resultant table of smallest distances
            best is the lower bound (exclusive) of node distance
            """
            while pq:
                cost, node = heappop(pq)
                # print("node", node, cost, dists[node])

                if cost > dists[node]:
                    continue

                q, r = divmod(cost, period)
                if r < change:
                    new_cost = cost + time
                else:
                    new_cost = period * (q + 1) + time

                for neighbor in neighbors[node]:
                    if best[neighbor] < new_cost < dists[neighbor]:
                        dists[neighbor] = new_cost
                        heappush(pq, (new_cost, neighbor))

        MAX = 10**8

        # In the first iteration, find the minimum to the dest
        dists = [MAX] * n
        dists[0] = 0
        best = [-1] * n
        pq = [(0, 0)]

        dijkstra(pq, best, dists)
        # print(dists)

        the_minimum = dists[n - 1]

        # the sequential iterations, (at most two),
        # start from any node, find the second minimum to each node (if possible),
        # If there are only two node in the graph,
        # three iterations in total are needed to find the second minimum of the last node.

        for rounds in range(2):
            # print("rounds", rounds)

            dists2 = [MAX] * n

            for src in range(n):
                pq = [(dists[src], src)]
                dijkstra(pq, dists, dists2)

            # print(dists2)
            second_min = dists2[-1]
            if second_min > the_minimum:
                return second_min

            dists = dists2

        raise AssertionError("unreachable")

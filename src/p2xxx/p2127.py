from leetcode_prelude import *


# Problem 2127
class Solution:
    def maximumInvitations(self, favorite: List[int]) -> int:
        from collections import defaultdict, deque

        degree = [0] * len(favorite)

        for dst in favorite:
            degree[dst] += 1

        tails = [0] * len(favorite)
        pending = deque([src for src, count in enumerate(degree) if count == 0])

        while pending:
            src = pending.popleft()
            dst = favorite[src]
            tails[dst] = max(tails[dst], tails[src] + 1)
            degree[dst] -= 1
            if degree[dst] == 0:
                pending.append(dst)

        # print(list(enumerate(tails)))

        maximum_two_legs = 0
        maximum_cyclic = 0

        for src in range(len(favorite)):
            if degree[src] == 0:
                continue

            # mark as visited
            degree[src] = 0
            cycle_list = [src]
            cycle = 1
            dst = src

            while True:
                node = favorite[dst]
                if node == src:
                    break
                cycle_list.append(node)
                # mark as visited
                degree[node] = 0
                cycle += 1
                dst = node

            # print("cycle", src, dst, cycle, cycle_list)

            assert cycle >= 2, cycle
            if cycle == 2:
                maximum_two_legs += 2 + tails[src] + tails[dst]
            else:
                maximum_cyclic = max(maximum_cyclic, cycle)

        return max(maximum_two_legs, maximum_cyclic)

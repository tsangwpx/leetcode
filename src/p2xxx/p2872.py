from leetcode_prelude import *


# Problem 2872
class Solution:
    def maxKDivisibleComponents(
        self,
        n: int,
        edges: List[List[int]],
        values: List[int],
        k: int,
    ) -> int:
        """
        toposort
        """

        if n == 1:
            assert values[0] % k == 0
            return 1

        from collections import deque

        links = [[] for _ in range(n)]
        for a, b in edges:
            links[a].append(b)
            links[b].append(a)

        count = 0
        leaves = deque([i for i, neighbors in enumerate(links) if len(neighbors) == 1])

        while leaves:
            for _ in range(len(leaves)):
                node = leaves.popleft()

                parents = links[node]
                if not parents:
                    # this is the final node
                    assert len(leaves) == 0
                    assert values[node] % k == 0
                    return count + 1

                assert len(parents) == 1, (node, parents)
                prev = parents[0]

                links[prev].remove(node)

                if len(links[prev]) == 1:
                    leaves.append(prev)

                if values[node] % k == 0:
                    count += 1
                else:
                    values[prev] += values[node]

        raise ValueError("the graph contain loop")

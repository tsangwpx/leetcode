from leetcode_prelude import *


# Problem 310
class Solution:
    def findMinHeightTrees(self, n: int, edges: List[List[int]]) -> List[int]:
        if n == 1:
            return [0]

        neighbors = [[] for _ in range(n)]

        for u, v in edges:
            neighbors[u].append(v)
            neighbors[v].append(u)

        connected = [len(s) for s in neighbors]
        # print(connected)

        leaves = [i for i, c in enumerate(connected) if c == 1]
        parents = []
        height = 0

        while leaves:
            # print("Leaf", leaves)
            # empty the parents list
            parents.clear()
            height += 1

            for node in leaves:
                for friend in neighbors[node]:
                    connected[friend] -= 1

                    # If friend node has one connectivity,
                    # it is the parent of this node and a leaf in next iteration
                    if connected[friend] == 1:
                        parents.append(friend)

            # swap them
            parents, leaves = leaves, parents
            # print("connected", connected)

        return parents

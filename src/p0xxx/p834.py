from leetcode_prelude import *


# Problem 834
class Solution:
    def sumOfDistancesInTree(
        self,
        n: int,
        edges: List[List[int]],
    ) -> List[int]:
        neighbors: list[list[int]] = [[] for _ in range(n)]

        for a, b in edges:
            neighbors[a].append(b)
            neighbors[b].append(a)

        # descendant counter
        counter = [0] * n

        # result
        res = [0] * n

        def dfs(node: int, parent: int):
            # pre order traversal
            for friend in neighbors[node]:
                if friend == parent:
                    continue

                dfs(friend, node)

                counter[node] += counter[friend]
                res[node] += res[friend] + counter[friend]

            counter[node] += 1

        def dfs2(node: int, parent: int):
            # post order traversal
            for friend in neighbors[node]:
                if friend == parent:
                    continue

                # switch the pivot from node to friend
                # counter[friend] nodes are closer to friend
                # counter[friend] nodes are farther to friend
                res[friend] = res[node] - counter[friend] + n - counter[friend]

                dfs2(friend, node)

        dfs(0, -1)

        # In pre-order visit, the result of root (node 0) is complete.
        # but the result of node farther from root is less and less complete.
        # So do a post-order visit to fix the issue from the root to leaves

        dfs2(0, -1)

        return res

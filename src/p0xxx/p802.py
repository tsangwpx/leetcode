from leetcode_prelude import *


# Problem 802
class Solution:
    def eventualSafeNodes(self, graph: List[List[int]]) -> List[int]:
        from collections import defaultdict

        n = len(graph)
        visited = [False] * n
        safe = [False] * n

        def dfs(node: int) -> None:
            if visited[node]:
                return safe[node]

            visited[node] = True

            destinations = graph[node]

            for dst in destinations:
                if not dfs(dst):
                    break
            else:
                safe[node] = True

            return safe[node]

        for i in range(n):
            dfs(i)

        return [i for i, s in enumerate(safe) if s]

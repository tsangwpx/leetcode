from leetcode_prelude import *


# Problem 1971
class Solution:
    def validPath(
        self,
        n: int,
        edges: List[List[int]],
        source: int,
        destination: int,
    ) -> bool:
        if source == destination:
            return True

        neighbors: list[list[int]] = [[] for _ in range(n)]

        for u, v in edges:
            neighbors[u].append(v)
            neighbors[v].append(u)

        visited = [False] * n
        pending = [source]
        visited[source] = True

        while pending:
            pivot = pending.pop()

            if destination in neighbors[pivot]:
                return True

            for friend in neighbors[pivot]:
                if visited[friend]:
                    continue
                visited[friend] = True
                pending.append(friend)

        return False

from leetcode_prelude import *


# Problem 2493
class Solution:
    def magnificentSets(
        self,
        n: int,
        edges: List[List[int]],
    ) -> int:
        from collections import deque

        adjacent = [[] for _ in range(n)]
        for a, b in edges:
            a -= 1
            b -= 1
            adjacent[a].append(b)
            adjacent[b].append(a)

        def make_partitions() -> list[list[int]]:
            visited: list[bool] = [False] * n
            partitions: list[set[int]] = []

            for i in range(n):
                if visited[i]:
                    continue

                visited[i] = True
                connected = [i]
                partitions.append(connected)

                pending = [i]

                while pending:
                    node = pending.pop()
                    for j in adjacent[node]:
                        if visited[j]:
                            continue
                        visited[j] = True
                        connected.append(j)
                        pending.append(j)

            return partitions

        def count_groups(start: int) -> int:
            layer = [None] * n

            depth = 0
            layer[start] = depth

            queue = deque()
            queue.append(start)

            while queue:
                for _ in range(len(queue)):
                    node = queue.popleft()

                    for friend in adjacent[node]:
                        if layer[friend] is None:
                            layer[friend] = depth + 1
                            queue.append(friend)
                        elif layer[friend] == depth:
                            return -1

                depth += 1

            return depth

        partitions = make_partitions()
        res = 0

        for nodes in partitions:
            count = max(count_groups(s) for s in nodes)
            if count < 0:
                return -1
            res += count

        return res

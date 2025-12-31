# Problem 684
class Solution:
    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
        from collections import defaultdict

        graph = defaultdict(set)
        for a, b in edges:
            graph[a].add(b)
            graph[b].add(a)

        pending = [k for k, v in graph.items() if len(v) == 1]
        while pending:
            top = pending.pop()
            friend = next(iter(graph.pop(top, ())))
            graph[friend].remove(top)

            if len(graph[friend]) == 1:
                pending.append(friend)

        # print(graph)
        for a, b in reversed(edges):
            if a in graph and b in graph[a]:
                return [a, b]

        raise AssertionError("unreachable?")

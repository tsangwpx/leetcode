from leetcode_prelude import *


# Problem 2097
class Solution:
    def validArrangement(self, pairs: List[List[int]]) -> List[List[int]]:
        from collections import defaultdict

        balance: dict[int, int] = defaultdict(int)
        edges: dict[int, list[int]] = defaultdict(list)

        start = stop = -1

        for start, stop in pairs:
            balance[start] -= 1
            balance[stop] += 1

            edges[start].append(stop)

        for node, count in balance.items():
            if count == 0:
                continue
            elif count == -1:
                assert start < 0, start
                start = node
            elif count == 1:
                assert stop < 0, stop
                stop = node
            else:
                assert False, (node, count)

        if start < 0 and stop < 0:
            start, _ = pairs[0]
            stop = start

        assert start >= 0 and stop >= 0, (start, stop)

        def dfs(rev_steps: list[int], edges: dict[int, list[int]], node: int) -> None:
            while edges[node]:
                dfs(rev_steps, edges, edges[node].pop())
            rev_steps.append(node)

        rev_steps = []
        dfs(rev_steps, edges, start)

        res = []
        for i in range(len(rev_steps) - 1, 0, -1):
            res.append((rev_steps[i], rev_steps[i - 1]))

        return res

    def validArrangement2(self, pairs: List[List[int]]) -> List[List[int]]:
        """Fool implementation that do not know placing node after dfs and reverse the walking steps"""
        from collections import defaultdict

        balance: dict[int, int] = defaultdict(int)
        edges: dict[int, list[int]] = defaultdict(list)

        for start, stop in pairs:
            balance[start] -= 1
            balance[stop] += 1

            edges[start].append(stop)

        start = stop = -1

        for node, count in balance.items():
            if count == 0:
                continue
            elif count == -1:
                assert start < 0, start
                start = node
            elif count == 1:
                assert stop < 0, stop
                stop = node
            else:
                assert False, (node, count)

        if start < 0 and stop < 0:
            start, _ = pairs[0]
            stop = start

        assert start >= 0 and stop >= 0, (start, stop)

        def walk(start: int, stop: int) -> None:
            node = start

            walk = defaultdict(list)

            while True:
                neighbors = edges[node]
                next_ = neighbors.pop()

                if not neighbors:
                    edges.pop(node)

                walk[node].append(next_)

                if next_ == stop:
                    break

                node = next_

            return walk

        def merge_graph(
            graph: dict[int, list[int]],
            extra: dict[int, list[int]],
        ) -> dict[int, list]:
            for node, steps in extra.items():
                steps = list(steps)
                steps.extend(graph.get(node, ()))
                graph[node] = steps

            return graph

        def print_graph(graph: dict[int, list[int]], start: int) -> None:

            # print(
            #     *[(node, list(steps)) for node, steps in graph.items()],
            #     sep="\n",
            # )

            indices = defaultdict(int)
            points: list[int] = [start]

            node = start
            while True:
                idx = indices[node]
                if idx >= len(graph.get(node, ())):
                    break
                indices[node] += 1
                next_ = graph[node][idx]
                points.append(next_)
                node = next_

            print("->".join(str(s) for s in points))

            for node, idx in indices.items():
                if idx != len(graph.get(node, ())):
                    assert False, f"Bad index {node} {idx}"

        graph = walk(start, stop)
        graph.setdefault(stop, [])
        # print_graph(graph, start)

        while edges:
            for node in edges.keys():
                if node in graph:
                    break
            else:
                assert False, "There must exist a node in the main graph"

            extra = walk(node, node)
            graph = merge_graph(graph, extra)
            # print_graph(graph, start)

        for steps in graph.values():
            steps.reverse()

        res = []
        node = start
        for _ in range(len(pairs)):
            next_ = graph[node].pop()
            res.append([node, next_])
            node = next_

        # print(*[(node, list(nexts)) for node, nexts in graph.items()], sep="\n")

        return res

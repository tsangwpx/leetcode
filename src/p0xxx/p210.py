from collections import defaultdict
from typing import List, Tuple


class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        # Kahn's algorithm
        inflows = [0] * numCourses
        flows = defaultdict(list)  # map src to its destinations

        for dst, src in prerequisites:
            # src -> dst
            inflows[dst] += 1
            flows[src].append(dst)

        # start with node with zero inflows
        pending = [i for i in range(numCourses) if inflows[i] == 0]
        order = []

        while pending:
            src = pending.pop()
            order.append(src)

            for dst in flows[src]:
                # remove an edge: src -> dst
                inflows[dst] -= 1
                if inflows[dst] == 0:
                    pending.append(dst)

        if list(filter(None, inflows)):
            return []

        return order


if __name__ == "__main__":
    print(Solution().findOrder(2, [[0, 0]]))
    print(Solution().findOrder(2, [[1, 0]]))
    print(Solution().findOrder(2, [[1, 0], [0, 1]]))
    print(Solution().findOrder(3, [[0, 1], [0, 2], [1, 2]]))

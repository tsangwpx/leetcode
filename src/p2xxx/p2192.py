from leetcode_prelude import *


# Problem 2192
class Solution:
    def getAncestors(self, n: int, edges: List[List[int]]) -> List[List[int]]:
        laters = [[] for _ in range(n)]
        formers = [[] for _ in range(n)]
        prerequisites = [0] * n

        for from_, to in edges:
            laters[from_].append(to)
            formers[to].append(from_)
            prerequisites[to] += 1

        memo = [None] * n
        res = [None] * n

        pending = [idx for idx, count in enumerate(prerequisites) if count == 0]

        while pending:
            node = pending.pop()

            ancestors = set(formers[node])

            for prev in formers[node]:
                ancestors.update(memo[prev])

            memo[node] = ancestors
            res[node] = [s for s in range(n) if s in ancestors]

            for next_ in laters[node]:
                prerequisites[next_] -= 1
                if prerequisites[next_] == 0:
                    pending.append(next_)

        return res

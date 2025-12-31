from leetcode_prelude import *


# Problem 1462
class Solution:
    def checkIfPrerequisite(
        self,
        numCourses: int,
        prerequisites: List[List[int]],
        queries: List[List[int]],
    ) -> List[bool]:
        """

        The function uses a depth-first search (DFS) approach to determine if one course is a prerequisite of another.
        It maintains a table to store the results of prerequisite checks to avoid redundant calculations.

        """
        table: list[list[bool | None]] = [
            [None] * numCourses for _ in range(numCourses)
        ]

        for i in range(numCourses):
            table[i][i] = False

        rev_depends = [[] for _ in range(numCourses)]
        for src, dst in prerequisites:
            table[src][dst] = True
            rev_depends[src].append(dst)

        def dfs(src: int, dst: int) -> bool:
            if table[src][dst] is not None:
                return table[src][dst]

            for child in rev_depends[src]:
                if dfs(child, dst):
                    result = True
                    break
            else:
                result = False

            table[src][dst] = result
            return result

        res = []
        for src, dst in queries:
            res.append(dfs(src, dst))

        return res

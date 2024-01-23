from collections import defaultdict
from typing import List, Tuple


class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        # Kahn's algorithm
        inflows = [0] * numCourses
        table = defaultdict(list)  # map src to its destinations

        for dst, src in prerequisites:
            # src -> dst
            inflows[dst] += 1
            table[src].append(dst)

        # start with node with zero inflows
        pending = [i for i in range(numCourses) if inflows[i] == 0]

        while pending:
            src = pending.pop()
            for dst in table[src]:
                # remove an edge: src -> dst
                inflows[dst] -= 1
                if inflows[dst] == 0:
                    pending.append(dst)

        return True

    def canFinish2(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        # Map course to its prerequisites
        prerequisite_table = defaultdict(set)

        for course, one_prerequisite in prerequisites:
            prerequisite_table[course].add(one_prerequisite)

        # 0 => unvisited, 1 => visited, -1 => visiting
        marks = [0] * numCourses

        for course in range(numCourses):
            if marks[course] > 0:
                continue

            stack = []

            while True:
                if marks[course] < 0:
                    return False
                elif marks[course] == 0:
                    stack.append((course, list(prerequisite_table.get(course, ()))))
                    marks[course] = -1
                else:
                    # course has visited, pop stack to get the next course
                    pass

                while stack:
                    top_course, top_prerequisites = stack[-1]
                    if top_prerequisites:
                        course = top_prerequisites.pop()
                        break
                    else:
                        stack.pop()
                        marks[top_course] = 1
                else:
                    break

        return True


if __name__ == '__main__':
    print(Solution().canFinish(2, [[0, 0]]))
    print(Solution().canFinish(2, [[1, 0]]))
    print(Solution().canFinish(2, [[1, 0], [0, 1]]))
    print(Solution().canFinish(3, [[0, 1], [0, 2], [1, 2]]))

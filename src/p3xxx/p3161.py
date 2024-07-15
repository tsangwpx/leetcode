from leetcode_prelude import *


# Problem 3161
class Solution:
    def getResults(self, queries: List[List[int]]) -> List[bool]:
        from sortedcontainers import SortedDict, SortedList

        LIMIT = 5 * 10**4

        obstacles = SortedList()
        obstacles.add(0)
        obstacles.add(LIMIT)

        intervals = SortedDict()
        intervals[LIMIT] = SortedList()
        intervals[LIMIT].add(0)

        # print(obstacles)
        # print(intervals)

        res = []

        for row in queries:
            if row[0] == 1:
                _, x = row
                if x == 0 or x == LIMIT:
                    # which are added already.
                    continue

                idx = obstacles.bisect_left(x)
                left = obstacles[idx - 1]
                right = obstacles[idx]

                # print("add", x, "left", left, "right", right)

                width = right - left

                intervals[width].remove(left)
                if len(intervals[width]) == 0:
                    intervals.pop(width)

                obstacles.add(x)

                left_width = x - left
                if left_width not in intervals:
                    intervals[left_width] = SortedList()
                intervals[left_width].add(left)

                right_width = right - x
                if right_width not in intervals:
                    intervals[right_width] = SortedList()
                intervals[right_width].add(x)

                # print(obstacles)
                # print(intervals)
            else:
                _, x, sx = row
                start = x - sx
                if start < 0:
                    res.append(False)
                    continue

                print("Query", x, sx, start)

                for width in intervals.irange(sx):
                    candidates: SortedList = intervals[width]
                    if not candidates:
                        continue

                    print("candidate", width, candidates)

                    found = False

                    idx = candidates.bisect_left(x - sx)
                    if idx > 0 or candidates[idx] == start:
                        res.append(True)
                        break

                else:
                    res.append(False)

        return res

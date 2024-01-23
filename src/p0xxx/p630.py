import operator
from heapq import heappush, heappop
from typing import List


class Solution:
    def scheduleCourse(self, courses: List[List[int]]) -> int:
        courses.sort(key=operator.itemgetter(1))

        days = 0
        pq = []

        for duration, last_day in courses:
            days += duration
            heappush(pq, -duration)

            if days > last_day:
                days += heappop(pq)

        return len(pq)


if __name__ == '__main__':
    print(Solution().scheduleCourse([[100, 200], [200, 1300], [1000, 1250], [2000, 3200]]))
    print(Solution().scheduleCourse([[1, 2]]))
    print(Solution().scheduleCourse([[3, 2], [4, 3]]))

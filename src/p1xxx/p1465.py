from typing import List


class Solution:
    def maxArea(self, h: int, w: int, horizontalCuts: List[int], verticalCuts: List[int]) -> int:
        def max_gap(size: int, cuts: List[int]) -> int:
            gap = 0
            last = 0
            for point in cuts:
                dist = point - last
                gap = max(gap, dist)
                last = point
            return max(gap, size - last)

        horizontalCuts.sort()
        verticalCuts.sort()

        return max_gap(h, horizontalCuts) * max_gap(w, verticalCuts) % (10 ** 9 + 7)


if __name__ == '__main__':
    print(Solution().scheduleCourse([[100, 200], [200, 1300], [1000, 1250], [2000, 3200]]))
    print(Solution().scheduleCourse([[1, 2]]))
    print(Solution().scheduleCourse([[3, 2], [4, 3]]))

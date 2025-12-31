from leetcode_prelude import *


# Problem 2064
class Solution:
    def minimizedMaximum(self, n: int, quantities: List[int]) -> int:
        def too_many(mid: int) -> bool:
            c = 0
            for quantity in quantities:
                c += -(-quantity // mid)

            return c > n

        left = 1
        right = 10**5

        while left < right:
            mid = (left + right) // 2

            if too_many(mid):
                left = mid + 1
            else:
                right = mid

        return left

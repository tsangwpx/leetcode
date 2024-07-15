from leetcode_prelude import *


# Problem 1518
class Solution:
    def numWaterBottles(self, numBottles: int, numExchange: int) -> int:
        count = numBottles
        empty = numBottles

        while empty >= numExchange:
            q, r = divmod(empty, numExchange)
            count += q
            empty = r + q

        return count

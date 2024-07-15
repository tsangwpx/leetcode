from leetcode_prelude import *


# Problem 3184 & Problem 3185
class Solution:
    def countCompleteDayPairs(self, hours: List[int]) -> int:
        from collections import defaultdict

        count = 0
        table = defaultdict(int)

        for h in hours:
            remainder = h % 24

            if remainder == 0:
                count += table[0]
                table[0] += 1
            else:
                friend = 24 - remainder
                count += table[friend]
                table[remainder] += 1

        return count

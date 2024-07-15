from leetcode_prelude import *


# Problem 974
class Solution:
    def subarraysDivByK(self, nums: List[int], k: int) -> int:
        table = [0] * k
        table[0] = 1

        acc = 0
        count = 0

        for number in nums:
            acc = (acc + number) % k
            count += table[acc]
            # print(number, acc, table[acc], count)
            table[acc] += 1

        return count

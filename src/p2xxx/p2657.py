from leetcode_prelude import *


# Problem 2657
class Solution:
    def findThePrefixCommonArray(self, A: List[int], B: List[int]) -> List[int]:
        n = len(A)
        counter = [0] * (n + 1)

        count = 0
        res = [0] * n

        for i, (a, b) in enumerate(zip(A, B)):
            counter[a] += 1
            counter[b] += 1

            if counter[a] == 2:
                count += 1

            if a != b and counter[b] == 2:
                count += 1

            res[i] = count

        return res

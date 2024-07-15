from leetcode_prelude import *


# Problem 1915
class Solution:
    def wonderfulSubstrings(self, word: str) -> int:
        counter = [0] * 1024
        counter[0] = 1
        count = 0

        acc = 0

        for ch in word:
            bit = 1 << (ord(ch) - ord("a"))
            acc ^= bit

            count += counter[acc]
            count += sum([counter[acc ^ (1 << i)] for i in range(10)])
            counter[acc] += 1

        return count

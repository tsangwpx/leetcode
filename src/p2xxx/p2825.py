from leetcode_prelude import *


# Problem 2825
class Solution:
    def canMakeSubsequence(self, str1: str, str2: str) -> bool:
        table = {chr(i + ord("a")): chr((i + 1) % 26 + ord("a")) for i in range(0, 26)}

        size = len(str2)
        left = 0

        for ch in str1:
            ch2 = table[ch]

            if str2[left] == ch or str2[left] == ch2:
                left += 1

            if left >= size:
                return True

        return False

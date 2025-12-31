from leetcode_prelude import *


# Problem 2696
class Solution:
    def minLength(self, s: str) -> int:
        r = ""

        for ch in s:
            r += ch

            while r[-2:] == "AB" or r[-2:] == "CD":
                r = r[:-2]

        return len(r)

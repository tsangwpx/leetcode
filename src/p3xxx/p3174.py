from leetcode_prelude import *


# Problem 3174
class Solution:
    def clearDigits(self, s: str) -> str:
        r = ""
        for ch in s:
            if ch in "0123456789":
                r = r[:-1]
            else:
                r += ch

        return r

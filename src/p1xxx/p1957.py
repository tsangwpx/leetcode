from leetcode_prelude import *


# Problem 1957
class Solution:
    def makeFancyString(
        self,
        s: str,
    ) -> str:
        r = ""

        for ch in s:
            if len(r) >= 2 and r[-1] == ch and r[-2] == ch:
                continue

            r += ch

        return r

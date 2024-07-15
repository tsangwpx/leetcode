from leetcode_prelude import *


# Problem 2716
class Solution:
    def minimizedStringLength(self, s: str) -> int:
        return len(set(s))

from leetcode_prelude import *


# Problem 2000
class Solution:
    def reversePrefix(self, word: str, ch: str) -> str:
        pos = word.find(ch)
        if pos >= 0:
            return word[pos::-1] + word[pos + 1 :]
        return word

from leetcode_prelude import *


# Problem 1408
class Solution:
    def stringMatching(self, words: List[str]) -> List[str]:
        res = []

        for word in words:
            for another in words:
                if word == another:
                    continue
                if word in another:
                    res.append(word)
                    break

        return res

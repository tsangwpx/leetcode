from leetcode_prelude import *


# Problem 1455
class Solution:
    def isPrefixOfWord(self, sentence: str, searchWord: str) -> int:
        for idx, word in enumerate(sentence.split(), 1):
            if word.startswith(searchWord):
                return idx

        return -1

from leetcode_prelude import *


# Problem 2108
class Solution:
    def firstPalindrome(self, words: List[str]) -> str:
        for word in words:
            if word == word[::-1]:
                return word

        return ""

    def firstPalindrome2(self, words: List[str]) -> str:
        for word in words:
            size = len(word)

            if (size & 1) == 0:
                left = word[0 : size // 2]
                right = word[size // 2 :][::-1]

                if left == right:
                    return word
            else:
                left = word[0 : size // 2]
                right = word[size // 2 + 1 :][::-1]

                if left == right:
                    return word

        return ""

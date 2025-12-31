from leetcode_prelude import *


# Problem 2559
class Solution:
    def vowelStrings(self, words: List[str], queries: List[List[int]]) -> List[int]:
        """
        Prefix sums

        Map words to boolean array whose items True
        if the corresponding words starting and ending with vowels.

        Build prefix sum with the boolean array.
        The query can be answered with the prefix sum.

        Note that the prefix sums are prepended with 0 so that
        the query to the first item (0, 0) is equivalent to pfxsum[1] - pfxsum[0]

        """

        vowels = "aeiou"

        count = 0

        pfxsum = [0] * (len(words) + 1)

        for idx, item in enumerate(words, 1):
            if item[0] in vowels and item[-1] in vowels:
                count += 1
            pfxsum[idx] = count

        res = []
        for left, right in queries:
            res.append(pfxsum[right + 1] - pfxsum[left])

        return res

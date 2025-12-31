from leetcode_prelude import *


# Problem 567
class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        from collections import Counter

        target = Counter(s1)
        window = Counter()
        left = 0

        for idx, ch in enumerate(s2):
            # Increase the freq counter of s2
            window[ch] += 1

            # Now, make sure that the sliding window of s2
            # is a subset of s1
            while left <= idx and window[ch] > target[ch]:

                window[s2[left]] -= 1
                left += 1

            # If the size of sliding window is exactly equal to s1,
            # it is a permutation of s1
            if (idx + 1 - left) == len(s1):
                return True

        return False

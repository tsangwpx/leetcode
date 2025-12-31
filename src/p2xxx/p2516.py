from leetcode_prelude import *


# Problem 2516
class Solution:
    def takeCharacters(self, s: str, k: int) -> int:
        """
        sliding window of untouched region

        1. the problem is equivalent to find the maximum size of the untouched string
        2. first, scan the string to count total frequencies by character
        3. move the sliding window from left to right while maximizing the window
            and maintain that the character frequencies >= k outside the window
        """
        from collections import Counter

        counter = Counter(s)
        if any(counter[ch] < k for ch in "abc"):
            return -1

        max_window = 0

        left = 0

        for right, ch in enumerate(s):
            counter[ch] -= 1

            while counter[ch] < k:
                counter[s[left]] += 1
                left += 1

            max_window = max(max_window, right + 1 - left)

            # print(left, right + 1, ch, sorted(counter.items()))

        return len(s) - max_window

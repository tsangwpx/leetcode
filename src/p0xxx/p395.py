# Problem 395
class Solution:
    def longestSubstring(self, string: str, min_count: int) -> int:
        from collections import Counter, defaultdict

        max_len = 0
        strlen = len(string)

        for kind_max in range(1, 26 + 1):
            counter = Counter()
            kind_count = 0
            left = 0
            idx = 0

            while left <= idx and idx < strlen:
                left2 = left
                idx2 = idx
                if kind_count > kind_max:
                    ch = string[left]
                    left2 = left2 + 1
                    counter[ch] -= 1
                    if counter[ch] == 0:
                        kind_count -= 1
                else:
                    ch = string[idx]
                    idx2 = idx + 1

                    counter[ch] += 1

                    if counter[ch] == 1:
                        kind_count += 1

                # the loop inside is O(1)
                # and it can be optimized by counting frequencies satisfying min_count
                if kind_count == kind_max and all(
                    v >= min_count or v == 0 for v in counter.values()
                ):
                    max_len = max(max_len, idx + 1 - left)

                left = left2
                idx = idx2

        return max_len

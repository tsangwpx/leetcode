from leetcode_prelude import *


# Problem 3020
class Solution:
    def maximumLength(self, nums: List[int]) -> int:
        from collections import Counter

        counter = Counter(nums)

        ones = counter.pop(1, 0)

        # 1's can form a pattern of itself
        if ones > 0:
            max_subset = ones - 1 if ones % 2 == 0 else ones
        else:
            max_subset = 0

        for number, count in counter.items():
            subset_len = 1

            while count >= 2:
                # print(number, count, subset_len)
                number = number**2
                count = counter.get(number, 0)
                if count >= 1:
                    subset_len += 1

            max_subset = max(max_subset, subset_len * 2 - 1)

        return max_subset

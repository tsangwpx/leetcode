from leetcode_prelude import *


# Problem 2364
class Solution:
    def countBadPairs(self, nums: List[int]) -> int:
        """
        j - i != nums[j] - nums[i]
        j - nums[j] != i - nums[i]
        """

        from collections import defaultdict

        counter = defaultdict(int)

        count = 0

        for j, number in enumerate(nums):
            k = j - number
            # j is the number of possible i
            # counter[k] is the number of correct i
            # j - counter[k] is the number of bad pairs
            count += j - counter[k]
            counter[k] += 1

        return count

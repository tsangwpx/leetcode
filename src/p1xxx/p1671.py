from leetcode_prelude import *


# Problem 1671
class Solution:
    def minimumMountainRemovals(
        self,
        nums: list[int],
    ) -> int:
        from bisect import bisect_left
        import itertools

        MAX_INT = 10**10  # very big int larger than possible value

        def lengths_of_increasing_subsequence(seq: list[int]) -> list[int]:
            """
            Return the longest lengths of subsequence at each index
            """
            lens = [0] * len(seq)
            # preallocate here so that the loop is less complicated
            dp = [MAX_INT] * len(seq)

            for i, item in enumerate(seq):
                idx = bisect_left(dp, item)
                dp[idx] = item
                lens[i] = idx + 1

            return lens

        lis = lengths_of_increasing_subsequence(nums)

        # reverse and reverse => lengths of decreasing subsequence
        lds = lengths_of_increasing_subsequence(nums[::-1])[::-1]

        # since there exists a solution, we use a big value and reduce it further
        min_removals = len(nums)

        for left, right in zip(lis, lds):
            if left >= 2 and right >= 2:
                min_removals = min(min_removals, len(nums) + 1 - left - right)

        return min_removals

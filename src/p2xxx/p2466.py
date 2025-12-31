from leetcode_prelude import *


# Problem 2466
class Solution:
    def countGoodStrings(
        self,
        low: int,
        high: int,
        zero: int,
        one: int,
    ) -> int:
        """
        DP

        Define dp[k] = the number of ways to form strings of size k.
        Therefore, dp[0] = 1 and
        initially, dp[zero] = dp[one] = 1 if zero != one
                    dp[zero] = 2 if zero == one

        Recursion equations are
        1. dp[k + one] += dp[k], and
        2. dp[k + zero] += dp[k]
        """
        MOD = 10**9 + 7

        limit = high + 1
        dp = [0] * limit

        dp[0] = 1

        for size in range(limit):
            size_zero = size + zero
            size_one = size + one

            if size_zero < limit:
                dp[size_zero] += dp[size]
                dp[size_zero] %= MOD

            if size_one < limit:
                dp[size_one] += dp[size]
                dp[size_one] %= MOD
        # print(dp)
        count = 0
        for i in range(low, high + 1):
            count += dp[i]
            count %= MOD

        return count

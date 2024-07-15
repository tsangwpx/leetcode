from leetcode_prelude import *


# Problem 552
class Solution:
    def checkRecord(self, n: int) -> int:
        MOD = 10**9 + 7
        dp = [[0, 0, 0] for _ in range(n)]

        # [P, L, A]
        dp[0] = [1, 1, 1]

        if n >= 2:
            dp[1] = [sum(dp[0]), 3, 2]

        if n >= 3:
            dp[2] = [sum(dp[1]), dp[1][2] + dp[1][0] + dp[0][2] + dp[0][0], 4]

        for i in range(3, n):
            dp[i][0] = sum(dp[i - 1]) % MOD
            dp[i][1] = (dp[i - 1][0] + dp[i - 1][2] + dp[i - 2][0] + dp[i - 2][2]) % MOD
            dp[i][2] = (dp[i - 1][2] + dp[i - 2][2] + dp[i - 3][2]) % MOD

        return sum(dp[-1]) % MOD

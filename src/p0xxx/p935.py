from leetcode_prelude import *


# Problem 935
class Solution:
    def knightDialer(self, n: int) -> int:
        MOD = 10**9 + 7
        dp = (1,) * 10

        for _ in range(1, n):
            a1, a2, a3, a4, a5, a6, a7, a8, a9, a0 = dp

            dp = (
                # 1, 2,3
                (a6 + a8) % MOD,
                (a7 + a9) % MOD,
                (a4 + a8) % MOD,
                # 4,5,6
                (a3 + a9 + a0) % MOD,
                0,
                (a1 + a7 + a0) % MOD,
                # 7,8,9
                (a2 + a6) % MOD,
                (a1 + a3) % MOD,
                (a4 + a2) % MOD,
                # 0
                (a4 + a6) % MOD,
            )

        return sum(dp) % MOD

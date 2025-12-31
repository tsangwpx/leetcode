from leetcode_prelude import *


# Problem 3317
class Solution:
    def numberOfWays(self, n: int, x: int, y: int) -> int:
        from functools import lru_cache

        cache_size = max(x, n) * 4
        MOD = 10**9 + 7

        limit = min(n, x) + 1

        def combination_table(n: int) -> list[int]:
            # compute the coefficients of [nC0, nC1, nC2, ..., nCn]
            dp0 = [0] * (n + 1)
            dp0[0] = 1
            dp1 = dp0.copy()

            for m in range(1, n + 1):
                mid = (m + 1) // 2

                dp1[0] = 1
                dp1[m] = 1

                for r in range(1, mid):
                    dp1[r] = dp1[m - r] = (dp0[r - 1] + dp0[r]) % MOD

                if m % 2 == 0:
                    dp1[mid] = (dp0[mid - 1] + dp0[mid]) % MOD

                dp0, dp1 = dp1, dp0

            return dp0

        def compute_factorials(n: int) -> list[int]:
            res = [1] * (n + 1)

            for i in range(2, n + 1):
                res[i] = res[i - 1] * i % MOD

            return res

        def compute_table(n: int) -> list[int]:
            if n == 1:
                return [0, 1]
            elif n == 2:
                return [0, 1, 2]

            prev = compute_table(n - 1)
            res = [0] * (n + 1)
            res[n] = factorials[n]

            for k in range(1, n):
                res[k] = k * (prev[k - 1] + prev[k]) % MOD

            return res

        ncr = combination_table(x)
        factorials = compute_factorials(n)
        table = compute_table(n)

        res = 0

        for k in range(1, limit):
            # Define H(n, k) the number of ways to distribute n performers into k stages
            # Each stage must have at least one performer, n >= k

            # print(k, ncr[k], table[k], pow(y, k, MOD))
            res = (res + ncr[k] * table[k] * pow(y, k, MOD)) % MOD

        return res

    def numberOfWays2(self, n: int, x: int, y: int) -> int:
        from functools import lru_cache

        MOD = 10**9 + 7

        cache_size = max(n, x) * 4

        @lru_cache(maxsize=cache_size)
        def ncr(n: int, r: int) -> int:
            if r > n:
                return 0
            elif r == 0 or r == n:
                return 1
            else:
                return (ncr(n - 1, r - 1) + ncr(n - 1, r)) % MOD

        @lru_cache(maxsize=cache_size)
        def factorial(n: int) -> int:
            if n == 0:
                return 1
            elif n <= 2:
                return n
            else:
                return n * factorial(n - 1)

        @lru_cache(maxsize=cache_size)
        def h(n: int, k: int) -> int:
            if n < k:
                return 0
            if k == 1:
                return 1
            elif n == k:
                return factorial(k)
            else:
                return (h(n - 1, k - 1) * k + h(n - 1, k) * k) % MOD

        res = 0

        for k in range(1, min(x, n) + 1):
            # Define H(n, k) the number of ways to distribute n performers into k stages
            # Each stage must have at least one performer, n >= k

            res = (res + ncr(x, k) * h(n, k) * pow(y, k, MOD)) % MOD

        return res

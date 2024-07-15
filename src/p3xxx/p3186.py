from leetcode_prelude import *


# Problem 3186
class Solution:
    def maximumTotalDamage(self, power: List[int]) -> int:
        from collections import Counter

        counter = Counter(power)
        keys = sorted(counter)

        dp = [[0] * len(keys) for _ in range(2)]

        left = -1

        for i in range(len(keys)):
            dmg = keys[i]
            count = counter[dmg]

            while keys[left + 1] < dmg - 2:
                left += 1

            if left >= 0:
                dp[0][i] = max(dp[0][i - 1], dp[1][i - 1])
                dp[1][i] = max(dp[0][left], dp[1][left]) + dmg * count
            else:
                dp[0][i] = max(dp[0][i - 1], dp[1][i - 1])
                dp[1][i] = dmg * count

        print(dp[0])
        print(dp[1])

        return max(dp[0][-1], dp[1][-1])

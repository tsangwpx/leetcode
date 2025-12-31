# Problem 494
class Solution:
    def findTargetSumWays(
        self,
        nums: List[int],
        target: int,
    ) -> int:
        from collections import Counter

        dp0 = Counter()
        dp0[0] = 1

        for number in nums:
            dp1 = Counter()

            for base, count in dp0.items():
                dp1[base + number] += count
                dp1[base - number] += count

            dp0.clear()
            dp0, dp1 = dp1, dp0

        return dp0[target]

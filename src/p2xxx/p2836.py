from leetcode_prelude import *


# Problem 2836
class Solution:
    def getMaxFunctionValue(self, receiver: List[int], k: int) -> int:
        """
        Binary lifting


        """

        max_power = 1
        while 2**max_power <= k:
            max_power += 1

        n = len(receiver)
        memo = [[(0, 0)] * max_power for _ in range(n)]

        # initialize 2**0 steps
        for idx in range(n):
            memo[idx][0] = (receiver[idx], receiver[idx])

        for power in range(1, max_power):
            prev = power - 1

            for idx in range(n):
                mid, score = memo[idx][prev]
                end, score2 = memo[mid][prev]
                memo[idx][power] = (end, score + score2)

        remaining = k
        step_powers = []
        for power in range(max_power - 1, -1, -1):
            step = 2**power
            if remaining >= step:
                remaining -= step
                step_powers.append(power)

        # print(memo)
        # print(step_powers)

        max_value = 0
        for idx in range(n):
            pos = idx
            total = idx

            for power in step_powers:
                pos, score = memo[pos][power]
                total += score

            max_value = max(max_value, total)

        return max_value

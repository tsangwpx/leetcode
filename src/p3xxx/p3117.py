from leetcode_prelude import *


# Problem 3117
class Solution:
    def minimumValueSum(self, nums: List[int], andValues: List[int]) -> int:
        and_len = len(andValues)

        ones = 2**30 - 1
        dp0 = {}
        dp0[(ones, 0)] = 0

        dp1 = {}

        for number in nums:
            for (bits, and_idx), value in dp0.items():
                if and_idx >= and_len:
                    continue

                bits &= number
                if bits >= andValues[and_idx]:
                    key = (bits, and_idx)

                    new_value = dp1.get(key)
                    if new_value is None:
                        new_value = value
                    else:
                        new_value = min(value, new_value)

                    dp1[key] = new_value

                if bits == andValues[and_idx]:
                    key = (ones, and_idx + 1)
                    new_value = dp1.get(key)
                    if new_value is None:
                        new_value = value + number
                    else:
                        new_value = min(new_value, value + number)
                    dp1[key] = new_value

            dp0, dp1 = dp1, dp0
            dp1.clear()

        return dp0.get((ones, and_len), -1)

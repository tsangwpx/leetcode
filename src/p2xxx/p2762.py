from leetcode_prelude import *


# Problem 2762
class Solution:
    def continuousSubarrays(self, nums: List[int]) -> int:
        """

        O(N) because left pointer can only access each position at most twice
        The "twice" here should be referenced to the difference condition of 2.
        So if the condition is changed to abs(a[i] - a[j]) <= k
        the complexity become O(Nk)
        """
        count = 0
        lower = upper = nums[0]
        left = 0

        for idx, number in enumerate(nums):
            upper = max(upper, number)
            lower = min(lower, number)

            if upper - lower > 2:
                win_len = idx - left
                count += win_len * (win_len + 1) // 2

                # reset the window
                left = idx
                lower = upper = number
                while left > 0 and abs(nums[left - 1] - number) <= 2:
                    left -= 1
                    upper = max(upper, nums[left])
                    lower = min(lower, nums[left])

                if left < idx:
                    # remove duplicate count in future
                    win_len = idx - left
                    count -= win_len * (win_len + 1) // 2

        win_len = len(nums) - left
        count += win_len * (win_len + 1) // 2

        return count

from leetcode_prelude import *


# Problem 3105
class Solution:
    def longestMonotonicSubarray(self, nums: List[int]) -> int:
        best = 1

        i = 0
        size = len(nums)

        while i < size:
            j = i + 1

            if j >= size:
                break

            if nums[j] == nums[i]:
                pass
            elif nums[j] > nums[i]:
                while j < size and nums[j] > nums[j - 1]:
                    j += 1
            elif nums[j] < nums[i]:
                while j < size and nums[j] < nums[j - 1]:
                    j += 1

            best = max(best, j - i)
            i = max(i + 1, j - 2)

        return best

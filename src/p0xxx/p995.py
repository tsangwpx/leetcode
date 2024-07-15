from leetcode_prelude import *


# Problem 995
class Solution:
    def minKBitFlips(self, nums: List[int], k: int) -> int:

        count = 0
        flipped = False
        length = len(nums)

        for i in range(len(nums)):
            if i >= k and nums[i - k] >= 2:
                flipped = not flipped

            if (not flipped and nums[i] == 0) or (flipped and nums[i] == 1):
                nums[i] += 2
                flipped = not flipped
                count += 1

                if i + k > length:
                    count = -1
                    break

        return count

from leetcode_prelude import *


# Problem 581
class Solution:
    def findUnsortedSubarray(self, nums: List[int]) -> int:
        """
        Alternatively, we may scan violation from left and from right
        find min and max between left and right if any,
        and expand and fix left and right according to min and max
        """
        for pos in range(1, len(nums)):
            if nums[pos - 1] > nums[pos]:
                break
        else:
            # already sorted
            return 0

        left = pos - 1
        right = pos
        min_ = nums[right]
        max_ = nums[left]

        # we may do bisect here
        while left >= 1 and nums[left - 1] > min_:
            left -= 1

        # print(left, right, min_, max_)

        # continue scanning violation while updating left and right
        for pos in range(pos + 1, len(nums)):
            value = nums[pos]

            if value >= max_ and value >= nums[pos - 1]:
                continue
            # print("issue", pos)

            right = pos
            max_ = max(max_, nums[pos - 1])
            min_ = min(min_, value)

            # we may do bisect here as well
            while left >= 1 and nums[left - 1] > min_:
                left -= 1

        # print(left, right)

        return right + 1 - left

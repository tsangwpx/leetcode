from leetcode_prelude import *


# Problem 962
class Solution:
    def maxWidthRamp(self, nums: List[int]) -> int:
        max_width = 0
        stack = []  # indexes whose values are decreasing

        for idx, number in enumerate(nums):
            if not stack or nums[stack[-1]] > number:
                stack.append(idx)

        for idx in range(len(nums))[::-1]:
            number = nums[idx]

            # print(max_width, idx, number, [(s, nums[s]) for s in stack])

            # in the equality case, i == j
            # the condition is True and the distance is 0
            # so no difference

            while stack and number >= nums[stack[-1]]:
                max_width = max(max_width, idx - stack.pop())

            if not stack:
                break

        return max_width

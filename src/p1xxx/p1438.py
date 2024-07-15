from leetcode_prelude import *


# Problem 1438
class Solution:
    def longestSubarray(self, nums: List[int], limit: int) -> int:
        from collections import deque

        removed = 0
        increasing = deque()
        decreasing = deque()

        # There are two similar solutions with two deque:
        # 1. Keep tracking the size of valid window.
        # 2. Keep tracking the numbers of added and removed elements (this solution).

        for idx in range(len(nums)):
            item = nums[idx]

            while increasing and increasing[-1] > item:
                increasing.pop()

            increasing.append(item)

            while decreasing and decreasing[-1] < item:
                decreasing.pop()

            decreasing.append(item)

            # This part is interesting.
            # Why use if instead of while?
            # If violation is found, longest size is unchanged,
            # (we add an element nums[idx] and remove one nums[removed])
            # and we need to maintain queue monotonicity
            #
            # If "while" is used, we keep removing elements until violation is gone,
            # which is the first approach
            if decreasing[0] - increasing[0] > limit:
                if nums[removed] == increasing[0]:
                    increasing.popleft()
                if nums[removed] == decreasing[0]:
                    decreasing.popleft()

                removed += 1

        # by comparing the total number of items and the number of removed items,
        # we know the size of the longest sub-array
        return len(nums) - removed

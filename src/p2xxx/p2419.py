from leetcode_prelude import *


# Problem 2419
class Solution:
    def longestSubarray(
        self,
        nums: List[int],
    ) -> int:
        maximum = 0
        running_length = 1
        max_length = 1

        prev = -1

        for idx, item in enumerate(nums):
            if item > maximum:
                # new maximum, reset the running length and the maximum length
                maximum = item
                running_length = 1
                max_length = 1
            elif item == maximum:
                # same maximum, bump running length if possible
                # update new the max length
                if item == prev:
                    running_length += 1
                else:
                    running_length = 1
                max_length = max(max_length, running_length)
            else:
                # otherwise, running_length is set to zero
                # but it is also handled in the equality case
                pass

            prev = item

        return max_length

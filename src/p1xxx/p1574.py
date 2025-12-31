from leetcode_prelude import *


# Problem 1574
class Solution:
    def findLengthOfShortestSubarray(self, arr: List[int]) -> int:
        """
        1. Find the leftmost and rightmost point violating the rule
        2. This give an upper bound of the answer
        3. Make a window between 0 and right, which are the deleted elements.
        4. Shrink the window from left and expand to the right while keep it small.
        """

        size = len(arr)

        left = -1
        for i in range(len(arr) - 1):
            if arr[i] > arr[i + 1]:
                left = i
                break

        if left < 0:
            return 0

        right = size - 1
        while right > left and arr[right - 1] <= arr[right]:
            right -= 1

        res = min(size - left - 1, right)

        i = 0
        j = right

        while i <= left and j < size:
            if arr[i] <= arr[j]:
                res = min(res, j - i - 1)
                i += 1
            else:
                j += 1

        return res

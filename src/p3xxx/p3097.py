from leetcode_prelude import *


# Problem 3097
class Solution:
    def minimumSubarrayLength(self, nums: List[int], k: int) -> int:
        """
        Observations:
        1. subarray is a continuos segment of the original array
        2. OR only add new bits
        3. Sliding window will work if we are able to remove old item in the OR value of the window
        4. Count bit occurrences. Update the OR value of the window if a bit is flipped
        """

        if k == 0:
            # special case
            return 1

        default = len(nums) + 1
        min_len = default

        size = len(nums)

        left = 0
        right = 0
        window = 0
        counter = [0] * 32

        def shift_list(n: int) -> list[int]:
            """a list of one bit shifts consist of the number"""
            shift = 0
            b = 1
            r = []
            while b <= n:
                if b & n == b:
                    r.append(shift)
                shift += 1
                b = 1 << shift
            return r

        while True:
            if window >= k and left < right:
                # try to shrink the window
                number = nums[left]
                left += 1
                for shift in shift_list(number):
                    counter[shift] -= 1
                    if counter[shift] == 0:
                        window &= ~(1 << shift)
            elif right < size:
                # extend the window if possible
                number = nums[right]
                right += 1
                for shift in shift_list(number):
                    counter[shift] += 1
                    if counter[shift] == 1:
                        window |= 1 << shift
            else:
                break

            # print(left, right, window, counter)

            if window >= k:
                min_len = min(min_len, right - left)

        if min_len == default:
            return -1
        else:
            return min_len

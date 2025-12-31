from leetcode_prelude import *


# Problem 2461
class Solution:
    def maximumSubarraySum(self, nums: List[int], k: int) -> int:
        """
        Sliding window between two pointers
        """
        max_sum = 0

        window = set()
        winsum = 0

        left = 0

        for idx, number in enumerate(nums):
            while number in window or (idx + 1 - left) > k:
                prev = nums[left]
                left += 1

                winsum -= prev
                window.remove(prev)

            window.add(number)
            winsum += number

            if len(window) >= k:
                max_sum = max(max_sum, winsum)

        return max_sum

    def maximumSubarraySum2(self, nums: List[int], k: int) -> int:
        from collections import Counter

        counter = Counter()
        window_sum = 0
        max_sum = 0
        distinct = 0

        for i, number in enumerate(nums):
            if i >= k:
                prev = nums[i - k]
                window_sum -= prev
                counter[prev] -= 1

                if counter[prev] == 1:
                    distinct += 1
                elif counter[prev] == 0:
                    distinct -= 1

            window_sum += number
            counter[number] += 1

            if counter[number] == 1:
                distinct += 1
            elif counter[number] == 2:
                distinct -= 1

            if distinct == k:
                max_sum = max(max_sum, window_sum)

        return max_sum

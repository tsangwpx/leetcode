from leetcode_prelude import *


# Problem 3209
class Solution:
    def countSubarrays(self, nums: List[int], k: int) -> int:
        print(nums)
        zero_bits = []

        for shift in range(32):
            bit = 1 << shift

            if (k & bit) == 0:
                zero_bits.append(bit)

        # print(zero_bits)

        prev_zeros = [0] * len(zero_bits)
        count = 0
        left = 0
        prev = 0
        acc = -1

        for i in range(len(nums)):
            acc &= nums[i]

            if acc >= k:
                for j, bit in enumerate(zero_bits):
                    if (nums[i] & bit) == 0:
                        prev_zeros[j] = i

            print(i, acc, prev, count)
            if acc == k:
                prev = min(prev_zeros)
                count += prev + 1 - left
            elif (acc & k) != k:
                left = i + 1
                acc = -1

        return count

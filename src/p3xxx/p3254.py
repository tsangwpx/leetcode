from leetcode_prelude import *


# Problem 3254
class Solution:
    def resultsArray(self, nums: List[int], k: int) -> List[int]:
        """
        1. Scan the array one by one and keep the number of consecutive increasing numbers
        2. Since the maximum is the current number, add it to result if count >= k. otherwise, -1
        3. Beware of one-off error about iteration steps and sub-array size
        """
        res = []

        count = 0
        last = -1

        for i in range(len(nums)):
            number = nums[i]

            if last + 1 == number:
                count += 1
            else:
                count = 1

            last = number

            if i + 1 >= k:
                if count >= k:
                    res.append(number)
                else:
                    res.append(-1)

        return res

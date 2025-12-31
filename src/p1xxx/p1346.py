from leetcode_prelude import *


# Problem 1346
class Solution:
    def checkIfExist(self, arr: List[int]) -> bool:
        seen = set()
        for number in arr:
            if number * 2 in seen or (number % 2 == 0 and number // 2 in seen):
                return True
            seen.add(number)

        return False


# Problem Q2
class Solution:
    def minBitwiseArray(self, nums: List[int]) -> List[int]:
        ans = []

        for number in nums:
            found = False
            for k in range(1, number):
                if (k | (k + 1)) == number:
                    found = True
                    ans.append(k)
                    break

            if not found:
                ans.append(-1)

        return ans


class Solution:
    def minBitwiseArray(self, nums: List[int]) -> List[int]:
        ans = []

        for number in nums:
            if number == 2:
                # special case
                ans.append(-1)
                continue

            shift = 0
            while ((number >> shift) & 1) == 1:
                shift += 1

            mask = (1 << shift) - 1
            prefix = number & ~mask

            target = prefix | (mask >> 1)
            ans.append(target)

        return ans

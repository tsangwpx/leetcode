from leetcode_prelude import *


# Problem 321
class Solution:
    def maxNumber(
        self,
        nums1: List[int],
        nums2: List[int],
        k: int,
    ) -> List[int]:

        def choose(arr: list[int], size: int) -> list[int]:
            """
            Choose digits from nums to form the largest number of a given size
            """
            n = len(arr)
            result = [0] * size
            j = 0  # stack pointer <= size

            for i, number in enumerate(arr):
                # how many digits are remaining
                rem = n - i

                while j > 0 and number > result[j - 1] < number and j + rem > size:
                    # Pop the last digit if its is smaller and we have enough digits to fill
                    j -= 1

                if j < size:
                    result[j] = number
                    j += 1

            return result

        def compare(arr1: list[int], arr2: list[int]) -> int:
            """
            Compare two numbers
            """

            for a, b in zip(arr1, arr2):
                cmp = a - b
                if cmp != 0:
                    return cmp

            return len(arr1) - len(arr2)

        def merge(arr1: list[int], arr2: list[int]) -> list[int]:
            """Merge two numbers and form a larger numbers"""
            i = 0
            j = 0
            result = []

            while i < len(arr1) and j < len(arr2):
                cmp = arr1[i] - arr2[j]
                if cmp == 0:
                    cmp = compare(arr1[i:], arr2[j:])

                if cmp > 0:
                    result.append(arr1[i])
                    i += 1
                else:
                    result.append(arr2[j])
                    j += 1

            # merge the remaining digits
            result.extend(arr1[i:])
            result.extend(arr2[j:])

            return result

        largest = [0] * k

        for m in range(
            max(0, k - len(nums2)),
            min(k + 1, len(nums1) + 1),
        ):
            arr1 = choose(nums1, m)
            arr2 = choose(nums2, k - m)
            number = merge(arr1, arr2)

            if compare(number, largest) > 0:
                largest = number

            # print(m, arr1, arr2, number, largest)

        return largest

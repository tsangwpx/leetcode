from leetcode_prelude import *


# Problem 2948
class Solution:
    def lexicographicallySmallestArray(
        self,
        nums: List[int],
        limit: int,
    ) -> List[int]:
        """

        Observations:
        1. the elements in the array can be grouped according to their distance.
            we sort the array first, and create group sequentially.
        2. within a group, we rearrange the numbers lexicographically.
            this is done since we add them from a sorted array.
        3. we need a table mapping a number to its group.
        4. To create the desired output array, iterating the input, for each number,
            remove the smallest item from the corresponding group,
            and append it to the output.
        5. we may optimize step 4 by reversing the lexicographical order in step 2
            so that list.pop() is used instead of list.pop(0)
        """

        from collections import Counter

        num2group: dict[int, int] = {}
        group: list[list[int]] = []
        counter = Counter(nums)

        prev = None

        for number, _ in sorted(counter.items(), reverse=True):
            if prev is None or prev - number > limit:
                group.append([])

            group[-1].append(number)
            num2group[number] = len(group) - 1
            prev = number

        res = [None] * len(nums)
        for i, number in enumerate(nums):
            idx = num2group[number]
            top = group[idx][-1]

            res[i] = top
            counter[top] -= 1
            if counter[top] == 0:
                group[idx].pop()

        return res

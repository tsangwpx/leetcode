from leetcode_prelude import *


# Problem 881
class Solution:
    def numRescueBoats(self, people: List[int], limit: int) -> int:
        people.sort(reverse=True)

        count = 0

        left = 0
        right = len(people) - 1

        while left < right:
            count += 1
            if people[left] + people[right] <= limit:
                right -= 1

            left += 1

        if left == right:
            count += 1

        return count

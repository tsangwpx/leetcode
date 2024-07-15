from leetcode_prelude import *


# Problem 2037
class Solution:
    def minMovesToSeat(self, seats: List[int], students: List[int]) -> int:
        seats.sort()
        students.sort()

        return sum([abs(a - b) for a, b in zip(seats, students)])

from leetcode_prelude import *


# Problem 1700
class Solution:
    def countStudents(self, students: List[int], sandwiches: List[int]) -> int:
        from collections import Counter

        preference_counter = Counter(students)

        for sandwich in sandwiches:
            if preference_counter[sandwich] == 0:
                break
            preference_counter[sandwich] -= 1

        return sum(preference_counter.values())

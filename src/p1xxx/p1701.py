from leetcode_prelude import *


# Problem 1701
class Solution:
    def averageWaitingTime(self, customers: List[List[int]]) -> float:
        duration = 0

        chief_time = 0

        for arrival_time, cooking_time in customers:

            if arrival_time >= chief_time:
                duration += cooking_time
                chief_time = arrival_time + cooking_time
            else:
                chief_time += cooking_time
                duration += chief_time - arrival_time

            # print(arrival_time, cooking_time, chief_time, duration)

        return duration / len(customers)

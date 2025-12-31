from leetcode_prelude import *


# Problem 1769
class Solution:
    def minOperations(self, boxes: str) -> List[int]:
        res = [0] * len(boxes)

        count = 0
        steps = 0
        for i in range(len(boxes)):
            res[i] += steps

            if boxes[i] == "1":
                count += 1

            steps += count

        print(res)

        count = 0
        steps = 0

        for i in range(len(boxes))[::-1]:
            res[i] += steps

            if boxes[i] == "1":
                count += 1

            steps += count

        print(res)

        return res

from leetcode_prelude import *


# Problem 846
class Solution:
    def isNStraightHand(self, hand: List[int], groupSize: int) -> bool:
        from collections import Counter

        size = len(hand)
        if size % groupSize != 0:
            return False

        counter = Counter(hand)

        for base in sorted(counter.keys()):
            count = counter[base]

            if count == 0:
                continue

            for delta in range(groupSize):
                number = base + delta
                counter[number] -= count

                if counter[number] < 0:
                    return False

        return set(counter.values()) == {0}

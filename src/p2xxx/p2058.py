from leetcode_prelude import *


# Problem 2058
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def nodesBetweenCriticalPoints(
        self,
        head: Optional[ListNode],
    ) -> List[int]:
        dist_min = 10**9
        dist_max = 0
        first = -1
        last = -1

        pos = 0
        prev = None
        node = head

        while node is not None:
            next_ = node.next

            if prev is not None and next_ is not None:
                minima = node.val < prev.val and node.val < next_.val
                maxima = node.val > prev.val and node.val > next_.val

                if minima or maxima:
                    if last >= 0:
                        dist_min = min(dist_min, pos - last)
                        dist_max = max(dist_max, pos - first)

                    last = pos

                    if first < 0:
                        first = pos

            # print(pos, last, dist_min, dist_max)

            prev = node
            node = next_
            pos += 1

        if dist_min >= 10**9:
            dist_min = -1

        if dist_max == 0:
            dist_max = -1

        return dist_min, dist_max

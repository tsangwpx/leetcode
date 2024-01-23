from leetcode_prelude import *


# Problem 2130
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def pairSum(self, head: Optional[ListNode]) -> int:
        values = []

        node = head
        while node is not None:
            values.append(node.val)
            node = node.next

        mid = len(values) // 2
        left = values[:mid]
        right = values[mid:]
        right.reverse()
        return max(a + b for a, b in zip(left, right))

from leetcode_prelude import *


# Problem 1669
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeInBetween(
        self, list1: ListNode, a: int, b: int, list2: ListNode
    ) -> ListNode:
        dummy = ListNode(next=list1)
        a_parent = dummy
        for _ in range(a):
            a_parent = a_parent.next

        b_node = a_parent.next
        for _ in range(b - a):
            b_node = b_node.next

        last_node = list2
        while last_node.next:
            last_node = last_node.next

        a_parent.next = list2
        last_node.next = b_node.next
        return dummy.next

from leetcode_prelude import *


# Problem 2181
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode()
        node = dummy

        sum_ = 0
        curr = head

        while curr is not None:
            sum_ += curr.val

            if curr.val == 0:
                if sum_ > 0:
                    node.next = ListNode(val=sum_)
                    node = node.next

                sum_ = 0

            curr = curr.next

        return dummy.next

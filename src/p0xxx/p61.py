from leetcode_prelude import *


# Problem 61
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def rotateRight(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        head = ListNode(next=head)
        count = 0
        node = head

        while node.next is not None:
            count += 1
            node = node.next

        k = k % count if count else 0
        if k == 0 or count == 0:
            return head.next

        skip = count - k
        tail = head
        for _ in range(skip):
            tail = tail.next

        res = tail.next
        tail.next = None
        node = res
        while node.next is not None:
            node = node.next
        node.next = head.next

        return res

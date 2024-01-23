from typing import Optional


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def partition(self, head: Optional[ListNode], x: int) -> Optional[ListNode]:
        left_head = left_tail = None
        right_head = right_tail = None

        while head is not None:
            node, head = head, head.next
            node.next = None

            if node.val < x:
                if left_head is None:
                    left_head = left_tail = node
                else:
                    left_tail.next = node
                    left_tail = node
            else:
                if right_head is None:
                    right_head = right_tail = node
                else:
                    right_tail.next = node
                    right_tail = node

        if left_head is None:
            left_head, right_head = right_head, None
        else:
            left_tail.next = right_head

        return left_head

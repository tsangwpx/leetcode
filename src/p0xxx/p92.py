from typing import Optional


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def reverseBetween(
        self, head: Optional[ListNode], left: int, right: int
    ) -> Optional[ListNode]:
        # the virtual head
        head = ListNode(0, head)

        fixed = head
        for _ in range(left - 1):
            fixed = fixed.next

        print("fixed", fixed.val)
        last = fixed.next
        prev, curr = last, last.next
        last.next = None  # unlink last

        for _ in range(right - left):
            print("point", curr.val, "to", prev.val)
            tmp = curr.next
            curr.next = prev
            prev = curr
            curr = tmp

        last.next = curr  # set last
        fixed.next = prev

        return head.next

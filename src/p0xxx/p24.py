from typing import List, Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# Problem 24
class Solution:
    def swapPairs(self, head: Optional[ListNode]) -> Optional[ListNode]:
        head = ListNode(next=head)

        node = head

        while node and node.next and node.next.next:
            left = node.next
            right = left.next

            node.next = right
            left.next = right.next
            right.next = left

            node = left

        return head.next

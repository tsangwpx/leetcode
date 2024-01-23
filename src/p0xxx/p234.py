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


# Problem 234
class Solution:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        count = 0
        node = head
        while node is not None:
            node = node.next
            count += 1

        split_at = count // 2 + count % 2 - 1

        node = head
        for _ in range(split_at):
            node = node.next

        # split a new linked list
        tail = node.next
        node.next = None

        # reverse the tail list
        prev_node = None
        curr_node = tail

        while curr_node is not None:
            # save the next node
            next = curr_node.next
            curr_node.next = prev_node
            prev_node = curr_node
            curr_node = next

        # the reverse link list is now at prev_node
        head2 = prev_node
        node = head
        node2 = head2

        while node is not None and node2 is not None:
            if node.val != node2.val:
                return False
            node = node.next
            node2 = node2.next

        return True

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


# Definition for a Node.
class Node:
    def __init__(self, x: int, next: "Node" = None, random: "Node" = None):
        self.val = int(x)
        self.next = next
        self.random = random


# Problem 443
class Solution:
    def compress(self, chars: List[str]) -> int:
        size = len(chars)
        wi = 0
        ri = 0

        while ri < size:
            ch = chars[ri]
            count = 1

            while ri + count < size and chars[ri + count] == ch:
                count += 1

            ri += count

            chars[wi] = ch
            wi += 1

            if count > 1:
                for ch in str(count):
                    chars[wi] = ch
                    wi += 1

        return wi

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


# Problem 1679
class Solution:
    def maxOperations(self, nums: List[int], k: int) -> int:
        from collections import Counter

        counter = Counter(nums)
        total_pairs = 0

        for a in list(counter.keys()):
            b = k - a
            a_count = counter.get(a, 0)

            if a_count == 0:
                continue

            if a == b:
                total_pairs += a_count // 2
            else:
                b_count = counter.get(b, 0)
                total_pairs += min(a_count, b_count)
                counter[a] = counter[b] = 0

        return total_pairs

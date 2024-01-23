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


# Problem 605
class Solution:
    def canPlaceFlowers(self, flowerbed: List[int], n: int) -> bool:
        size = len(flowerbed)

        for i, filled in enumerate(flowerbed):
            if filled:
                continue
            if i > 0 and flowerbed[i - 1]:
                continue
            if i + 1 < size and flowerbed[i + 1]:
                continue
            flowerbed[i] = 1
            n -= 1
            if n <= 0:
                break

        return n <= 0

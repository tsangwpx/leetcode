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


# Problem 80
class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        wi = 0
        last = None

        for ri, number in enumerate(nums):
            if number == last:
                count += 1
            else:
                count = 1

            if count <= 2:
                if ri != wi:
                    nums[wi] = number
                wi += 1

            last = number

        return wi

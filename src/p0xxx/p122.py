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


# Problem 122
class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        owned = [0] * len(prices)
        unowned = owned.copy()

        # probably it is good idea to buy the stock on day 1.
        owned[0] = -prices[0]

        for day in range(1, len(prices)):
            price = prices[day]

            owned[day] = max(owned[day - 1], unowned[day - 1] - price)
            unowned[day] = max(unowned[day - 1], owned[day] + price)

        return unowned[-1]

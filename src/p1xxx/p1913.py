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


# Problem 1913
class Solution:
    def maxProductDifference(self, nums: List[int]) -> int:
        from heapq import heapify, heappop

        neg_nums = [-s for s in nums]

        heapify(nums)
        heapify(neg_nums)

        return heappop(nums) * heappop(nums) - heappop(neg_nums) * heappop(neg_nums)

    def maxProductDifference2(self, nums: List[int]) -> int:
        from heapq import heapify, heappushpop

        heap_min = [nums[0], nums[1]]
        heap_min.sort()

        heap_max = [
            -heappushpop(heap_min, nums[2]),
            -heappushpop(heap_min, nums[3]),
        ]
        heap_max.sort()

        for number in nums[4:]:
            number = heappushpop(heap_min, number)
            _ = heappushpop(heap_max, -number)

        return heap_min[0] * heap_min[1] - heap_max[0] * heap_max[1]

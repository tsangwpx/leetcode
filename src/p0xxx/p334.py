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


# Problem 334
class Solution:
    def increasingTriplet(self, nums: List[int]) -> bool:
        smaller = small = 2**32

        for number in nums:
            # print(smaller, small, number)
            if number <= smaller:
                smaller = number
            elif number <= small:
                small = number
            else:
                return True

        return False

    def increasingTriplet2(self, nums: List[int]) -> bool:
        size = len(nums)
        seen_min_i = 2**32
        seen_min_j = 2**32

        for i in range(0, size - 2):
            if nums[i] >= seen_min_i:
                continue

            seen_min_i = min(seen_min_i, nums[i])

            for j in range(i + 1, size - 1):
                if nums[j] <= seen_min_i:
                    continue

                if nums[j] >= seen_min_j:
                    continue

                seen_min_j = min(seen_min_j, nums[j])

                for k in range(j + 1, size):
                    if nums[j] < nums[k]:
                        return True

        return False

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


# Problem 2353


from heapq import heapify, heappop, heappush

from sortedcontainers import SortedSet


class FoodRatings:
    def __init__(self, foods: List[str], cuisines: List[str], ratings: List[int]):
        food_table = {}
        cuisine_table = {}

        for food, cuisine, rating in zip(foods, cuisines, ratings):
            food_table[food] = (rating, cuisine)

            if cuisine in cuisine_table:
                sorted_foods = cuisine_table[cuisine]
            else:
                sorted_foods = SortedSet()
                cuisine_table[cuisine] = sorted_foods

            sorted_foods.add((-rating, food))

        self._food_table = food_table
        self._cuisine_table = cuisine_table

    def changeRating(self, food: str, newRating: int) -> None:
        food_table = self._food_table
        rating, cuisine = food_table[food]

        if rating == newRating:
            return None

        food_table[food] = (newRating, cuisine)

        sorted_foods = self._cuisine_table[cuisine]
        sorted_foods.discard((-rating, food))
        sorted_foods.add((-newRating, food))

    def highestRated(self, cuisine: str) -> str:
        sorted_foods = self._cuisine_table[cuisine]
        return sorted_foods[0][1]


# Your FoodRatings object will be instantiated and called as such:
# obj = FoodRatings(foods, cuisines, ratings)
# obj.changeRating(food,newRating)
# param_2 = obj.highestRated(cuisine)

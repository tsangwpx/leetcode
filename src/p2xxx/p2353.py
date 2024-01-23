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


class FoodRatings:
    def __init__(self, foods: List[str], cuisines: List[str], ratings: List[int]):
        food_table = {}
        cuisine_table = {}

        for food, cuisine, rating in zip(foods, cuisines, ratings):
            food_table[food] = (rating, cuisine)

            cuisine_pq, cuisine_foods = cuisine_table.setdefault(cuisine, ([], []))
            cuisine_pq.append((-rating, food))
            cuisine_foods.append(food)

        for cuisine_pq, cuisine_foods in cuisine_table.values():
            heapify(cuisine_pq)
            cuisine_foods.sort()

        self._food_table = food_table
        self._cuisine_table = cuisine_table

    def changeRating(self, food: str, newRating: int) -> None:
        food_table = self._food_table
        rating, cuisine = food_table[food]

        if rating == newRating:
            return None

        food_table[food] = (newRating, cuisine)
        cuisine_pq, _ = self._cuisine_table[cuisine]

        heappush(cuisine_pq, (-newRating, food))

    def highestRated(self, cuisine: str) -> str:
        food_table = self._food_table
        cuisine_pq, cuisine_foods = self._cuisine_table[cuisine]

        neg_rating, food = cuisine_pq[0]
        if -neg_rating == food_table[food][0]:
            return food

        # the heap need refresh
        if len(cuisine_pq) >= len(cuisine_foods) * 4:
            # rebuild the heap from scratch if too large
            cuisine_pq[:] = [(-food_table[food][0], food) for food in cuisine_foods]
            heapify(cuisine_pq)
        else:
            while True:
                neg_rating, food = cuisine_pq[0]
                if -neg_rating == food_table[food][0]:
                    break
                # pop entry when its rating is out of date
                heappop(cuisine_pq)

        return cuisine_pq[0][1]


# Your FoodRatings object will be instantiated and called as such:
# obj = FoodRatings(foods, cuisines, ratings)
# obj.changeRating(food,newRating)
# param_2 = obj.highestRated(cuisine)

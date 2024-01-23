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


# Problem 134
class Solution:
    def canCompleteCircuit(self, gas: List[int], cost: List[int]) -> int:
        stations = len(gas)
        start = 0
        gas_balance = 0

        while start < stations:
            where = start
            tank = 0  # reset the fuel tank

            while where < stations:
                # refill gas
                tank += gas[where] - cost[where]
                gas_balance += gas[where] - cost[where]

                if tank < 0:
                    break

                where += 1

            if tank < 0:
                # If out of fuel, this means we cant start between "start" and "where"
                # So, next is where + 1
                # If where + 1 is out of bounds, out of fuel is destined and
                # gas_balance must be negative.
                start = where + 1
            else:
                # otherwise, we have tried all gas station and this start is the answer.
                break

        if gas_balance >= 0:
            # If there is enough fuel for a complete cycle,
            # start is the smallest start index.
            return start
        else:
            # otherwise, it is impossible
            return -1

    def canCompleteCircuit2(self, gas: List[int], cost: List[int]) -> int:
        jumps = [(0, s) for s in gas]

        stations = len(gas)

        for start in range(stations - 1, -1, -1):
            pos = start
            fuel = 0
            dest = start + stations

            while True:
                where = pos % stations
                dist_change, fuel_change = jumps[where]
                pos += dist_change
                fuel += fuel_change

                if pos >= dest:
                    return start

                where = pos % stations

                if fuel < cost[where]:
                    break

                fuel -= cost[where]
                pos += 1

            jumps[start] = (pos - start, fuel)
            # print(start, pos, fuel)

        # print(jumps)

        return -1

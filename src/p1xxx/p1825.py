# Problem 1825
from sortedcontainers import SortedList

from leetcode_prelude import *


class MKAverage:
    """
    Implement with SortedList (aka. BTree)

    If implementing with heap, the stale removed entries stay forever.
    It seem like a memory leak.
    """

    def __init__(self, m: int, k: int):
        from collections import deque

        self.m = m
        self.k = k
        self._total = 0
        self._count = 0
        self._window = deque()
        self._front = SortedList()
        self._main = SortedList()
        self._back = SortedList()

    def addElement(self, num: int) -> None:
        if len(self._window) == self.m:
            item = self._window.popleft()

            if self._front and self._front[-1] >= item:
                self._front.remove(item)
            elif self._back and self._back[0] <= item:
                self._back.remove(item)
            else:
                self._main.remove(item)
                self._total -= item

        self._window.append(num)
        self._front.add(num)

        if len(self._front) > self.k:
            item = self._front.pop()
            self._main.add(item)
            self._total += item

        if self._main and self._main[0] < self._front[-1]:
            # swap main[0] and front[-1] if violate ordering
            small = self._main.pop(0)
            self._total -= small

            large = self._front.pop(-1)

            self._main.add(large)
            self._total += large

            self._front.add(small)

        if len(self._back) < self.k and self._main:
            item = self._main.pop()
            self._total -= item
            self._back.add(item)

        if self._main and self._back and self._main[-1] > self._back[0]:
            # swap main[-1] and back[-1] if violate ordering
            large = self._main.pop(-1)
            self._total -= large

            small = self._back.pop(0)

            self._main.add(small)
            self._total += small

            self._back.add(large)

    def calculateMKAverage(self) -> int:
        if len(self._window) < self.m:
            return -1

        return self._total // (self.m - self.k * 2)


# Your MKAverage object will be instantiated and called as such:
# obj = MKAverage(m, k)
# obj.addElement(num)
# param_2 = obj.calculateMKAverage()

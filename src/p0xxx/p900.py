from leetcode_prelude import *


# Problem 900
class RLEIterator:

    def __init__(self, encoding: List[int]):
        self._encoding = encoding
        self._index = 0

    def next(self, n: int) -> int:
        length = len(self._encoding)

        while self._index < length:
            delta = min(self._encoding[self._index], n)
            self._encoding[self._index] -= delta
            n -= delta

            if n == 0:
                return self._encoding[self._index + 1]

            if self._encoding[self._index] == 0:
                self._index += 2

        return -1


# Your RLEIterator object will be instantiated and called as such:
# obj = RLEIterator(encoding)
# param_1 = obj.next(n)

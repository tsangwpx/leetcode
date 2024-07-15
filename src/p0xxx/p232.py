from leetcode_prelude import *


# Problem 232
class MyQueue:
    def __init__(self):
        self._ready = []
        self._stack = []

    def push(self, x: int) -> None:
        self._stack.append(x)

    def _prepare_ready(self):
        if not self._ready:
            while self._stack:
                self._ready.append(self._stack.pop())

    def pop(self) -> int:
        self._prepare_ready()
        return self._ready.pop()

    def peek(self) -> int:
        self._prepare_ready()
        return self._ready[-1]

    def empty(self) -> bool:
        return not self._ready and not self._stack


# Your MyQueue object will be instantiated and called as such:
# obj = MyQueue()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.peek()
# param_4 = obj.empty()

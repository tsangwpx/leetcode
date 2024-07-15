# Problem 1172
from heapq import heappop, heappush

from leetcode_prelude import *


class DinnerPlates:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.stacks: list[list[int]] = []
        self.free = []

    def push(self, val: int) -> None:
        found = False

        if self.free:
            idx = heappop(self.free)

            if idx >= len(self.stacks):
                # No free stack is available
                del self.free[:]
            else:
                found = True

        if not found:
            idx = len(self.stacks)
            self.stacks.append([])
            found = True

        stack = self.stacks[idx]
        stack.append(val)

        if len(stack) < self.capacity:
            heappush(self.free, idx)

        print("push", self.stacks, self.free)

    def pop(self) -> int:
        if not self.stacks:
            return -1

        idx = len(self.stacks) - 1
        val = self.popAtStack(idx)
        return val

    def popAtStack(self, index: int) -> int:
        if index >= len(self.stacks):
            return -1

        stack = self.stacks[index]
        if not stack:
            return -1

        full = len(stack) == self.capacity
        val = stack.pop()

        while self.stacks and not self.stacks[-1]:
            self.stacks.pop()

        if full and index < len(self.stacks):
            # Add the stack to free heap
            heappush(self.free, index)

        print("pop at", index, self.stacks, self.free)
        return val


# Your DinnerPlates object will be instantiated and called as such:
# obj = DinnerPlates(capacity)
# obj.push(val)
# param_2 = obj.pop()
# param_3 = obj.popAtStack(index)

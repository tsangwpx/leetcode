from leetcode_prelude import *


# Problem 1115
class FooBar:
    def __init__(self, n):
        from threading import Condition

        self.n = n
        self._foo_turn = True
        self._cond = Condition()

    def foo(self, printFoo: "Callable[[], None]") -> None:

        for i in range(self.n):

            with self._cond:
                self._cond.wait_for(lambda: self._foo_turn)
                printFoo()
                self._foo_turn = False
                self._cond.notify()

    def bar(self, printBar: "Callable[[], None]") -> None:

        for i in range(self.n):
            with self._cond:
                self._cond.wait_for(lambda: not self._foo_turn)
                printBar()
                self._foo_turn = True
                self._cond.notify()

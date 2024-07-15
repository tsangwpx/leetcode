# Problem 1912
from heapq import heappop, heappush

from leetcode_prelude import *


class MovieRentingSystem:
    """
    Since tree algo is slow in Python,
    this implementation use a heap with a state counter
    to indicate the latest state, and ignore stale entries.
    But this may lead to memory leak because stale entries
    are not immediately removed from other heap.

    Alternatively, use sorted containers.
    """

    def __init__(self, n: int, entries: List[List[int]]):
        # (shop, movie) -> int
        self._prices = {}

        # (shop, movie) -> int
        self._item_states = {}

        # movie -> (price, shop, state)
        self._movie_queues: dict[int, list[tuple[int, int, int]]] = {}

        # (price, shop, movie, state)
        self._rented_heap: list[tuple[int, int, int, int]] = []

        for shop, movie, price in entries:
            key = (shop, movie)
            self._item_states[key] = 0
            self._prices[key] = price

            heap = self._movie_queues.setdefault(movie, [])
            heappush(heap, (price, shop, 0))

    def search(self, movie: int) -> List[int]:
        heap = self._movie_queues.get(movie)
        if heap is None:
            return []
        tmp = []

        while heap and len(tmp) < 5:
            row = heappop(heap)
            _, shop, state = row

            key = (shop, movie)
            state2 = self._item_states[key]

            if state != state2:
                # stale entry
                continue

            tmp.append(row)

        for row in tmp:
            heappush(heap, row)

        return [shop for _, shop, _ in tmp]

    def rent(self, shop: int, movie: int) -> None:
        key = (shop, movie)
        price = self._prices[key]

        self._item_states[key] += 1
        state = self._item_states[key]

        heappush(self._rented_heap, (price, shop, movie, state))

    def drop(self, shop: int, movie: int) -> None:
        key = (shop, movie)
        price = self._prices[key]

        self._item_states[key] += 1
        state = self._item_states[key]

        heap = self._movie_queues[movie]
        heappush(heap, (price, shop, state))

    def report(self) -> List[List[int]]:
        heap = self._rented_heap

        tmp = []

        while heap and len(tmp) < 5:
            row = heappop(heap)
            _, shop, movie, state = row

            key = (shop, movie)
            state2 = self._item_states[key]

            if state != state2:
                # stale entry
                continue

            tmp.append(row)

        for row in tmp:
            heappush(heap, row)

        return [[shop, movie] for _, shop, movie, _ in tmp]


# Your MovieRentingSystem object will be instantiated and called as such:
# obj = MovieRentingSystem(n, entries)
# param_1 = obj.search(movie)
# obj.rent(shop,movie)
# obj.drop(shop,movie)
# param_4 = obj.report()

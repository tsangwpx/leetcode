# Problem 773
class Solution:
    @staticmethod
    def _precompute() -> dict[tuple[int, int, int, int, int, int], int]:
        from heapq import heappop, heappush

        table: dict[tuple[int, int, int, int, int, int], int] = {}
        pq = []

        table[(1, 2, 3, 4, 5, 0)] = 0
        heappush(pq, (0, (1, 2, 3, 4, 5, 0)))

        while pq:
            moves, (c0, c1, c2, c3, c4, c5) = heappop(pq)

            next_boards = []

            if c0 == 0 or c1 == 0:
                next_boards.append((c1, c0, c2, c3, c4, c5))

            if c1 == 0 or c2 == 0:
                next_boards.append((c0, c2, c1, c3, c4, c5))

            if c3 == 0 or c4 == 0:
                next_boards.append((c0, c1, c2, c4, c3, c5))

            if c4 == 0 or c5 == 0:
                next_boards.append((c0, c1, c2, c3, c5, c4))

            if c0 == 0 or c3 == 0:
                next_boards.append((c3, c1, c2, c0, c4, c5))

            if c1 == 0 or c4 == 0:
                next_boards.append((c0, c4, c2, c3, c1, c5))

            if c2 == 0 or c5 == 0:
                next_boards.append((c0, c1, c5, c3, c4, c2))

            for next_ in next_boards:
                next_move = table.get(next_)
                if next_move is None or next_move > moves + 1:
                    table[next_] = moves + 1
                    heappush(pq, (moves + 1, next_))

        # print(len(table))
        return table

    _TABLE = _precompute()

    def slidingPuzzle(self, board: List[List[int]]) -> int:
        board = tuple(board[0] + board[1])
        return self._TABLE.get(board, -1)

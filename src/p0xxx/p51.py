import collections
from typing import Deque, List, Tuple


class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        # if n == 1:
        #     return [["Q"]]

        nsq = n**2
        heads = [i // n * n for i in range(nsq)]
        # print('heads', n, heads)
        ok = []

        def place(table: List[int], pos: int):
            row, col = divmod(pos, n)

            for i in range(heads[pos], heads[pos] + n):
                table[i] = 1

            for i in range(pos, nsq, n):
                table[i] = 1

            # diagonal
            for i in range(pos, min(pos + (n - col) * (n + 1), nsq), n + 1):
                table[i] = 1

            for i in range(pos, min(pos + (col + 1) * (n - 1), nsq), n - 1):
                table[i] = 1

        # testing = [0] * nsq
        # place(testing, 3)
        # print('testing', testing)

        pending: Deque[Tuple[List[int], List[int]]] = collections.deque()
        pending.append(([], [0] * nsq))

        # print('pending', n, pending)
        while pending:
            locations, board = pending.popleft()
            start = heads[len(locations) * n]
            stop = start + n
            # print('loc', n, locations, start, stop, board)

            for i in range(start, stop):
                if board[i]:
                    continue

                next_locations = locations[:]
                next_locations.append(i)
                # print('next', len(next_locations), next_locations)

                if len(next_locations) == n:
                    ok.append(next_locations)
                    continue

                next_board = board[:]
                place(next_board, i)

                pending.append((next_locations, next_board))

        # print('ok', ok)

        strings = ["%sQ%s" % ("." * i, "." * (n - 1 - i)) for i in range(n)]
        # print(n, 'strings', strings)

        return [[strings[s % n] for s in cfg] for cfg in ok]


def run(nums):
    soln = Solution()
    return soln.solveNQueens(nums)


def main():
    inputs = [
        (
            1,
            [["Q"]],
        ),
        (
            2,
            [],
        ),
        (
            3,
            [],
        ),
        (
            4,
            [[".Q..", "...Q", "Q...", "..Q."], ["..Q.", "Q...", "...Q", ".Q.."]],
        ),
        (
            5,
            [
                ["Q....", "..Q..", "....Q", ".Q...", "...Q."],
                ["Q....", "...Q.", ".Q...", "....Q", "..Q.."],
                [".Q...", "...Q.", "Q....", "..Q..", "....Q"],
                [".Q...", "....Q", "..Q..", "Q....", "...Q."],
                ["..Q..", "Q....", "...Q.", ".Q...", "....Q"],
                ["..Q..", "....Q", ".Q...", "...Q.", "Q...."],
                ["...Q.", "Q....", "..Q..", "....Q", ".Q..."],
                ["...Q.", ".Q...", "....Q", "..Q..", "Q...."],
                ["....Q", ".Q...", "...Q.", "Q....", "..Q.."],
                ["....Q", "..Q..", "Q....", "...Q.", ".Q..."],
            ],
        ),
        (6, []),
        (
            7,
            [],
        ),
        (
            8,
            [],
        ),
        (
            9,
            None,
        ),
    ]

    for n, expected in inputs:
        actual = run(n)
        print("result", len(actual), actual)


if __name__ == "__main__":
    main()

from leetcode_prelude import *


# Problem 752
class Solution:
    def openLock(
        self,
        deadends: List[str],
        target: str,
    ) -> int:
        from collections import deque

        deadends: set[str] = set(deadends)

        if "0000" in deadends:
            return -1

        digit_neighbours = {
            str(k): (str((k - 1) % 10), str((k + 1) % 10)) for k in range(10)
        }

        deadends.add("0000")
        queue = deque()
        queue.append((0, "0000"))

        while queue:
            steps, passcode = queue.popleft()

            if passcode == target:
                return steps

            partitions = (
                ("", passcode[0], passcode[1:4]),
                (passcode[0:1], passcode[1], passcode[2:4]),
                (passcode[0:2], passcode[2], passcode[3:4]),
                (passcode[0:3], passcode[3], ""),
            )

            neighbours = []

            for left, digit, right in partitions:
                prev_digit, next_digit = digit_neighbours[digit]
                neighbours.append(left + prev_digit + right)
                neighbours.append(left + next_digit + right)

            for next_passcode in neighbours:
                if next_passcode not in deadends:
                    deadends.add(next_passcode)
                    queue.append((steps + 1, next_passcode))

        return -1

    def openLock2(
        self,
        deadends: List[str],
        target: str,
    ) -> int:
        from heapq import heappop, heappush

        visited: set[str] = set(deadends)

        if "0000" in visited:
            return -1

        if target == "0000":
            return 0

        visited.add("0000")
        pq = [(0, "0000")]

        while pq:
            # print(pq)
            dist, passcode = heappop(pq)

            table: tuple[tuple[str, str, str]] = (
                ("", passcode[0], passcode[1:4]),
                (passcode[0:1], passcode[1], passcode[2:4]),
                (passcode[0:2], passcode[2], passcode[3:4]),
                (passcode[0:3], passcode[3], ""),
            )

            new_passcodes = []

            for left, digit, right in table:
                if digit == "0":
                    new_passcodes.append(left + "1" + right)
                    new_passcodes.append(left + "9" + right)
                elif digit == "9":
                    new_passcodes.append(left + "0" + right)
                    new_passcodes.append(left + "8" + right)
                else:
                    digit = int(digit)
                    new_passcodes.append(left + str(digit + 1) + right)
                    new_passcodes.append(left + str(digit - 1) + right)

            if target in new_passcodes:
                return dist + 1

            for new_passcode in new_passcodes:
                if new_passcode not in visited:
                    visited.add(new_passcode)
                    heappush(pq, (dist + 1, new_passcode))

        return -1

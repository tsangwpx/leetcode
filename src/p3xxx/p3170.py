from leetcode_prelude import *


# Problem 3170
class Solution:
    def clearStars(self, s: str) -> str:
        valid = [True] * len(s)

        table: dict[str, list[int]] = {
            chr(ch): [] for ch in range(ord("a"), ord("z") + 1)
        }

        for idx, ch in enumerate(s):
            if ch == "*":
                valid[idx] = False

                for indices in table.values():
                    if indices:
                        removed = indices.pop()
                        valid[removed] = False
                        break

            else:
                indices = table[ch]
                indices.append(idx)

        return "".join([ch for idx, ch in enumerate(s) if valid[idx]])

    def clearStars2(self, s: str) -> str:
        from heapq import heappop, heappush

        pq = []

        for i, ch in enumerate(s):
            if ch == "*":
                heappop(pq)
            else:
                heappush(pq, (ch, -i))

        pq.sort(key=lambda pair: -pair[1])

        return "".join(ch for ch, _ in pq)

from leetcode_prelude import *


# Problem 512
class Solution:
    def findRotateSteps(self, ring: str, key: str) -> int:
        from collections import defaultdict

        alphabet_indices: dict[str, list[int]] = defaultdict(list)

        for idx, ch in enumerate(ring):
            alphabet_indices[ch].append(idx)

        pending: dict[int, int] = {
            0: 0,
        }

        for idx, ch in enumerate(key):
            new_pending = {}

            for pos, steps in pending.items():
                for next_pos in alphabet_indices[ch]:
                    # we may pre-compute the dist as well
                    normalized = (next_pos - pos) % len(ring)
                    dist = min(normalized, len(ring) - normalized)

                    new_pending[next_pos] = min(
                        new_pending.get(next_pos, 10**9),
                        steps + dist + 1,
                    )

            pending = new_pending
            # print(pending)

        return min(pending.values())

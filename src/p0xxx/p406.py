from typing import List, Tuple, Optional


class Solution:
    def reconstructQueue(self, people: List[List[int]]) -> List[List[int]]:
        people.sort()
        size = len(people)
        queue: List[Optional[List[int]]] = [None] * len(people)
        # print('people', people)

        lasts = {}

        pos = 0  # first available slot
        for h, k in people:
            i = lasts.get(h, pos)
            if i == pos:
                prev_k = 0  # this is the first of the kind
            else:
                _, prev_k = queue[i]

            skip = k - prev_k  # number of None or taller item to skip
            while i < size and skip > 0:
                if queue[i] is None or queue[i][0] >= h:
                    skip -= 1
                i += 1
            while i < size and queue[i] is not None:
                i += 1
            queue[i] = [h, k]
            lasts[h] = i

            pos_origin = pos
            if pos == i:
                pos += 1
                while pos < size and queue[pos] is not None:
                    pos += 1
            # print(pos_origin, pos, i, (h, k), queue)

        return queue


if __name__ == '__main__':
    print(Solution().reconstructQueue([[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]]))
    print(Solution().reconstructQueue([[2, 4], [3, 4], [9, 0], [0, 6], [7, 1], [6, 0], [7, 3], [2, 5], [1, 1], [8, 0]]))
    print(Solution().reconstructQueue([[6, 0], [5, 0], [4, 0], [3, 2], [2, 2], [1, 4]]))

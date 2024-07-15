from leetcode_prelude import *


# Problem 2402
class Solution:
    def mostBooked(self, n: int, meetings: List[List[int]]) -> int:
        from heapq import heapify, heappop, heappush, heapreplace

        meetings.sort()

        counter = [0] * n

        unused = list(range(n))

        running = []  # (finished_time, room)

        for start, stop in meetings:
            # Release rooms when meetings are completed
            while running and start >= running[0][0]:
                _, room = heappop(running)
                heappush(unused, room)

            if unused:
                room = heappop(unused)
                counter[room] += 1
                heappush(running, (stop, room))
            else:
                # If no room is availabe,
                # this meeting would start once a previous meeting is completed.
                finished_time, room = running[0]
                finished_time += stop - start
                counter[room] += 1
                heapreplace(running, (finished_time, room))

        max_count = max(counter)
        return counter.index(max_count)

    def mostBooked2(self, n: int, meetings: List[List[int]]) -> int:
        from heapq import heapify, heappop, heappush

        counter = [0] * n

        unused = list(range(n))
        heapify(unused)

        running = []  # (finished_time, room)
        pending = [(a, b) for a, b in meetings]
        heapify(pending)

        time = 0

        while running or pending:
            # Release rooms when meetings are completed
            while running and time >= running[0][0]:
                _, room = heappop(running)
                heappush(unused, room)

            # Allocate if both room and meeting are available
            while unused and pending and time >= pending[0][0]:
                room = heappop(unused)
                counter[room] += 1

                start, stop = heappop(pending)
                heappush(running, (time + stop - start, room))

            # print(time, unused, running, pending)

            # Advance time
            if unused:
                # If there is free room, new time is
                # 1. next start of meeting if any
                # 2. next stop of meeting if any
                # 3. or break
                if pending:
                    # Q: Can time >= pending[0][0]?
                    # A: No! because we should have used all ready meetings.

                    time = pending[0][0]
                elif running:
                    time = running[0][0]
                else:
                    # end of loop
                    break
            else:
                # All room is used, skip to next completd time
                time = running[0][0]

        room = 0
        for i in range(1, n):
            if counter[i] > counter[room]:
                room = i

        return room

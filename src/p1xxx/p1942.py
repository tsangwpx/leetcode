from leetcode_prelude import *


# Problem 1942
class Solution:
    def smallestChair(
        self,
        times: List[List[int]],
        targetFriend: int,
    ) -> int:
        """
        Simulate events and return the seat id in interest
        """
        from heapq import heapify, heappop, heappush

        events = []

        for friend, (arrival, leaving) in enumerate(times):
            events.append((arrival, 1, friend))
            events.append((leaving, 0, friend))

        events.sort()

        allocations: dict[int, int] = {}
        next_seat_id = 0
        empty_seats = []

        for _, is_arrival, friend in events:
            if is_arrival:
                if empty_seats:
                    seat_id = heappop(empty_seats)
                    allocations[friend] = seat_id
                else:
                    seat_id = next_seat_id
                    next_seat_id += 1
                    allocations[friend] = seat_id

                if friend == targetFriend:
                    return seat_id
            else:
                seat_id = allocations.pop(friend)
                heappush(empty_seats, seat_id)

        raise AssertionError("unreachable")

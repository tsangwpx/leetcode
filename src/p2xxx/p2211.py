from leetcode_prelude import *


# Problem 2211
class Solution:
    def countCollisions(self, directions: str) -> int:
        stripped = directions.lstrip("L").rstrip("R")
        return stripped.count("L") + stripped.count("R")

    def countCollisions2(self, directions: str) -> int:
        collisions = 0
        prev = "L"
        right = 0

        for car in directions:
            match car:
                case "L":
                    if prev == "R":
                        collisions += 1 + right
                        right = 0
                        prev = "S"
                    elif prev == "S":
                        assert right == 0
                        collisions += 1

                case "R":
                    prev = "R"
                    right += 1

                case "S":
                    prev = "S"
                    collisions += right
                    right = 0

                case _:
                    raise AssertionError()

            # print(collisions)

        return collisions

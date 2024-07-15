from typing import List


class Solution:
    def candy(self, ratings: List[int]) -> int:
        size = len(ratings)
        if size == 1:
            return 1

        candies = [1] * len(ratings)
        children = list(range(len(ratings)))
        children.sort(key=lambda s: ratings[s])

        for index in children:
            my_rating = ratings[index]
            my_candies = candies[index]

            if index > 0 and my_rating > ratings[index - 1]:
                my_candies = max(my_candies, candies[index - 1] + 1)

            if index + 1 < size and my_rating > ratings[index + 1]:
                my_candies = max(my_candies, candies[index + 1] + 1)

            candies[index] = my_candies

        return sum(candies)


def main():
    print(Solution().isPossible([1, 1000000000]))
    # return
    print(Solution().isPossible([2, 4, 6, 7, 8]))
    print(Solution().isPossible([1, 2, 3, 4, 5]))
    print(Solution().isPossible([9, 3, 5]))
    print(Solution().isPossible([1, 1, 1, 2]))
    print(Solution().isPossible([8, 5]))


if __name__ == "__main__":
    main()

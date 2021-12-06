from __future__ import annotations
from typing import List, Sequence

import numpy as np

INPUT_FILENAME = "input.txt"
MAX_DAYS_BEFORE_BIRTH = 8


class LanternFishSimulator:
    def __init__(self, counts: List[int]) -> None:
        self.counts = counts
        self.day = 0

    @classmethod
    def from_input(cls, timers: Sequence[int]) -> LanternFishSimulator:
        counts = np.zeros(MAX_DAYS_BEFORE_BIRTH + 1, dtype=int)
        inicies, initial_counts = np.unique(timers, return_counts=True)
        counts[inicies] = initial_counts

        return LanternFishSimulator(counts=counts.tolist())

    @property
    def n_fish(self) -> int:
        return sum(self.counts)

    def simulate(self, n_days: int) -> None:
        counts = self.counts
        for _ in range(n_days):
            new_fish = counts.pop(0)
            counts[6] += new_fish
            counts.append(new_fish)

        self.day += n_days
        self.counts = counts


def read_input(filename: str) -> Sequence[int]:

    return np.loadtxt(filename, delimiter=",", dtype=int)


if __name__ == "__main__":
    initial_fish_timers = read_input(INPUT_FILENAME)
    N_DAYS_PART_1 = 80
    N_DAYS_PART_2 = 256

    print("Part1:")
    simulator = LanternFishSimulator.from_input(timers=initial_fish_timers)
    simulator.simulate(n_days=N_DAYS_PART_1)
    n_fish_part_1 = simulator.n_fish
    print(f"There are {n_fish_part_1} lanternfish after {N_DAYS_PART_1} days.")

    print("\nPart2:")
    simulator.simulate(n_days=N_DAYS_PART_2 - N_DAYS_PART_1)
    n_fish_part_2 = simulator.n_fish
    print(f"There are {n_fish_part_2} lanternfish after {N_DAYS_PART_2} days.")

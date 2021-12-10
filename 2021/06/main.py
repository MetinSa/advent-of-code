from __future__ import annotations
from dataclasses import dataclass
from typing import List, Sequence

import numpy as np

INPUT_FILENAME = "input.txt"
MAX_DAYS_BEFORE_BIRTH = 8


@dataclass
class LanternfishSimulator:
    """Lanternfish simulator class.

    Attributes
    ----------
    fish_counts
        List where index i represents i days until birthing, and value
        represents number of fish with i days until birthing.
    current_day
        The current day in the simulation.
    """

    fish_counts: List[int]
    current_day: int = 0

    @classmethod
    def from_ages(cls, ages: Sequence[int]) -> LanternfishSimulator:
        counts = np.zeros(MAX_DAYS_BEFORE_BIRTH + 1, dtype=int)
        indicies, initial_counts = np.unique(ages, return_counts=True)
        counts[indicies] = initial_counts

        return LanternfishSimulator(fish_counts=counts.tolist())

    @property
    def total_fish_count(self) -> int:
        return sum(self.fish_counts)

    def simulate(self, n_days: int) -> None:
        fish_counts = self.fish_counts

        for _ in range(n_days):
            # We delete the first entry of the list which corresponds to
            # birthing fish, and append this number to the end of the list
            # which then represents the newly born fish with 8 days left
            # before birthing. Additionally, we increment the counts at index
            # 6, which corresponds to newly birthed fish by the number of
            # birthing fish.
            new_fish = fish_counts.pop(0)
            fish_counts[6] += new_fish
            fish_counts.append(new_fish)

        self.current_day += n_days
        self.fish_counts = fish_counts


def get_initial_ages(filename: str) -> Sequence[int]:
    return np.loadtxt(filename, delimiter=",", dtype=int)


if __name__ == "__main__":
    initial_fish_timers = get_initial_ages(INPUT_FILENAME)
    N_DAYS_PART_1 = 80
    N_DAYS_PART_2 = 256

    print("Part1:")
    simulator = LanternfishSimulator.from_ages(ages=initial_fish_timers)
    simulator.simulate(n_days=N_DAYS_PART_1)
    n_fish_part_1 = simulator.total_fish_count
    print(f"There are {n_fish_part_1} lanternfish after {N_DAYS_PART_1} days.")

    print("\nPart2:")
    simulator.simulate(n_days=N_DAYS_PART_2 - N_DAYS_PART_1)
    n_fish_part_2 = simulator.total_fish_count
    print(f"There are {n_fish_part_2} lanternfish after {N_DAYS_PART_2} days.")

from dataclasses import dataclass, field
from typing import List

import numpy as np


MAX_VALUE = 9


def read_grid(filename: str) -> np.ndarray:
    grid: List[List[int]] = []
    with open(filename, "r") as file:
        for line in file:
            line = line.strip()
            grid.append([int(number) for number in line])

    return np.array(grid)


@dataclass
class Simulator:
    grid: np.ndarray
    step: int = 0
    total_flashes: int = 0
    simultaneous_flash_steps: List[int] = field(default_factory=list)

    def simulate_step(self) -> None:
        self.step += 1
        grid = self.grid.copy() + 1
        padded_grid = np.pad(grid, pad_width=(1, 1))

        flashes = np.argwhere(grid > 9).tolist()
        self.total_flashes += len(flashes)

        while flashes:
            new_flashes = []
            for i, j in flashes:
                sliding_window_view = padded_grid[i : i + 3, j : j + 3]
                extra_flashes = (sliding_window_view <= MAX_VALUE) == (
                    sliding_window_view + 1 > MAX_VALUE
                )
                if indicies := np.argwhere(extra_flashes).tolist():
                    for ii, jj in indicies:
                        new_flashes.append([ii - 1 + i, jj - 1 + j])
                        self.total_flashes += 1
                padded_grid[i : i + 3, j : j + 3] += 1
            flashes = new_flashes

        padded_grid[padded_grid > MAX_VALUE] = 0
        self.grid = padded_grid[1:-1, 1:-1]
        if (self.grid == 0).all():
            self.simultaneous_flash_steps.append(self.step)


if __name__ == "__main__":
    INPUT_FILE = "input.txt"
    grid = read_grid(INPUT_FILE)

    print("Part 1:")
    simulator_part1 = Simulator(grid)
    for i in range(100):
        simulator_part1.simulate_step()
    print(
        f"There were {simulator_part1.total_flashes} after {simulator_part1.step} steps."
    )

    print("\nPart 2:")
    simulator_part2 = Simulator(grid)
    while True:
        simulator_part2.simulate_step()
        if simulator_part2.simultaneous_flash_steps:
            break
    print(f"The first step during which all octopuses flashes is {simulator_part2.step}")

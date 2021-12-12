from dataclasses import dataclass
from typing import List, Optional

import numpy as np


MAX_VALUE = 9


def read_octopus_grid(filename: str) -> np.ndarray:
    grid: List[List[int]] = []
    with open(filename, "r") as file:
        for line in file:
            line = line.strip()
            grid.append([int(number) for number in line])

    return np.array(grid)


@dataclass
class OctopusFlashSimulator:
    octopus_grid: np.ndarray
    step: int = 0
    total_flashes: int = 0
    simultaneous_flash_step: Optional[int] = None

    def simulate_step(self) -> None:
        self.step += 1
        grid = self.octopus_grid.copy() + 1
        padded_grid = np.pad(grid, pad_width=(1, 1))

        flashes = np.argwhere(grid > 9).tolist()
        self.total_flashes += len(flashes)

        while flashes:
            new_flashes = []

            for row, col in flashes:
                sliding_window_view = padded_grid[row : row + 3, col : col + 3]
                additional_flashes = (sliding_window_view <= MAX_VALUE) == (
                    sliding_window_view + 1 > MAX_VALUE
                )

                additional_flash_indices = np.argwhere(additional_flashes).tolist()
                for extra_row, extra_col in additional_flash_indices:
                    new_flashes.append([extra_row - 1 + row, extra_col - 1 + col])
                    self.total_flashes += 1

                padded_grid[row : row + 3, col : col + 3] += 1
            flashes = new_flashes

        padded_grid[padded_grid > MAX_VALUE] = 0
        self.octopus_grid = padded_grid[1:-1, 1:-1]

        if self.simultaneous_flash_step is None and (self.octopus_grid == 0).all():
            self.simultaneous_flash_step = self.step


if __name__ == "__main__":
    INPUT_FILE = "input.txt"
    grid = read_octopus_grid(INPUT_FILE)

    print("Part 1:")
    simulator_part1 = OctopusFlashSimulator(grid)
    for i in range(100):
        simulator_part1.simulate_step()
    print(
        f"There were {simulator_part1.total_flashes} after {simulator_part1.step} steps."
    )

    print("\nPart 2:")
    simulator_part2 = OctopusFlashSimulator(grid)
    while True:
        simulator_part2.simulate_step()
        if simulator_part2.simultaneous_flash_step is not None:
            break
    print(
        f"The first step during which all octopuses flashes is {simulator_part2.step}"
    )

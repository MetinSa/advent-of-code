from __future__ import annotations
from dataclasses import dataclass, astuple
from typing import List

import numpy as np

INPUT_FILENAME = "input.txt"


@dataclass
class Line:
    x0: int
    y0: int
    x1: int
    y1: int

    def is_horizontal_or_vertical(self) -> bool:
        is_horizontal = self.x0 == self.x1
        is_vertical = self.y0 == self.y1

        return np.add(is_horizontal, is_vertical)


class Grid:
    def __init__(self, grid_len: int) -> None:
        self.grid = np.zeros((grid_len + 1, grid_len + 1))

    @classmethod
    def from_horizontal_or_vertical_lines(cls, lines: List[Line]) -> Grid:
        grid = Grid(grid_len=get_max_value_from_lines(lines))
        for line in lines:
            if line.is_horizontal_or_vertical():
                grid.add_line(line=line)

        return grid

    @classmethod
    def from_lines(cls, lines: List[Line]) -> Grid:
        grid = Grid(grid_len=get_max_value_from_lines(lines))
        for line in lines:
            grid.add_line(line=line)

        return grid

    def add_line(self, line: Line) -> None:
        x_indicies = (
            np.arange(line.x0, line.x1 + 1)
            if line.x0 < line.x1
            else np.arange(line.x0, line.x1 - 1, -1)
        )
        y_indicies = (
            np.arange(line.y0, line.y1 + 1)
            if line.y0 < line.y1
            else np.arange(line.y0, line.y1 - 1, -1)
        )

        self.grid[y_indicies, x_indicies] += 1

    def get_n_overlaps(self) -> int:

        return np.sum(self.grid > 1)


def read_lines(filename: str) -> List[Line]:
    lines: List[Line] = []

    with open(filename, "r") as file:
        for line in file:
            line = line.strip().replace(" -> ", ",").split(",")
            x0, y0, x1, y1 = (int(coord) for coord in line)
            lines.append(Line(x0, y0, x1, y1))

    return lines


def get_max_value_from_lines(lines: List[Line]) -> int:
    coords = (astuple(line)[2:] for line in lines)

    return max(max(coords, key=lambda item: max(item)))


if __name__ == "__main__":
    lines = read_lines(INPUT_FILENAME)

    grid1 = Grid.from_horizontal_or_vertical_lines(lines=lines)
    print("Part1:")
    print(grid1.grid)
    print(f"has {grid1.get_n_overlaps()} overlaps.")

    grid2 = Grid.from_lines(lines=lines)
    print("Part2:")
    print(grid2.grid)
    print(f"has {grid2.get_n_overlaps()} overlaps.")

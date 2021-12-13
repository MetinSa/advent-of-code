from typing import Dict, List, Tuple

import numpy as np


InstructionsType = Tuple[List[Tuple[int, int]], Dict[str, List[int]]]


def read_instructions(filename: str) -> InstructionsType:
    coordinates: List[Tuple[int, int]] = []
    folds: Dict[str, List[int]] = {}

    with open(filename, "r") as file:
        for line in file:
            line = line.strip()
            if not line:
                continue
            if line[0].isnumeric():
                x, y = line.split(",")
                coordinates.append((int(x), int(y)))
            else:
                equation = line.split()[-1]
                axis, value = equation.split("=")
                if axis not in folds:
                    folds[axis] = []
                folds[axis].append(int(value))

    return coordinates, folds


def get_grid(coordinates: List[Tuple[int, int]]) -> np.ndarray:
    max_x = max(coordinates, key=lambda x: x[0])[0]
    max_y = max(coordinates, key=lambda x: x[1])[1]
    shape = (max_y + 1, max_x + 1)
    grid = np.zeros(shape, dtype=bool)

    for coord in coordinates:
        x, y = coord
        grid[y, x] = True

    return grid


def get_folded_grid(
    grid: np.ndarray,
    folds: Dict[str, List[int]],
    break_at_first_fold: bool = False,
) -> np.ndarray:

    AXIS_MAPPINGS = {"x": 1, "y": 0}
    for xy, foldings in folds.items():
        axis = AXIS_MAPPINGS[xy]
        for folding in foldings:
            grid = np.delete(grid, folding, axis=axis)
            first_split, second_split = np.split(grid, [folding], axis=axis)
            second_split = np.flip(second_split, axis=axis)
            grid = first_split + second_split
            if break_at_first_fold:
                return grid

    return grid


def visualize_grid(grid: np.ndarray) -> None:
    grid = np.array(folded_grid, dtype=str)
    grid[grid == "True"] = "#"
    grid[grid == "False"] = "."
    for row in grid:
        print("".join(row))


if __name__ == "__main__":
    INPUT_FILE = "input.txt"
    coordinates, folds = read_instructions(filename=INPUT_FILE)
    grid = get_grid(coordinates=coordinates)

    print("Part 1:")
    folded_grid = get_folded_grid(grid=grid, folds=folds, break_at_first_fold=True)
    visible_dots = folded_grid.sum()
    print(f"There are {visible_dots} visible dots after the first folding.")

    print("\nPart 2:")
    folded_grid = get_folded_grid(grid=grid, folds=folds)
    print("The code on the folded paper reads:")
    visualize_grid(grid=folded_grid)

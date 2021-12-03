from typing import Tuple


INPUT_FILENAME = "input.txt"


def get_position_from_file(filename: str) -> Tuple[int, int]:
    """Returns the depth and horizontal position given a file with input.

    Parameters
    ----------
    filename
        Name of the input file.

    Returns
    -------
    depth
        Integer representing the final depth.
    horizontal_position
        Integer representing the final horizontal position.
    """

    depth = 0
    horizontal_position = 0

    with open(filename, "r") as file:
        for line in file:
            input, value = line.split()
            value = int(value)
            if input == "forward":
                horizontal_position += value
            elif input == "down":
                depth += value
            elif input == "up":
                depth -= value

    return depth, horizontal_position


def get_position_with_aim_from_file(filename: str) -> Tuple[int, int]:
    """Returns the depth and horizontal position (using aim) given a file with input.

    Parameters
    ----------
    filename
        Name of the input file.

    Returns
    -------
    depth
        Integer representing the final depth.
    horizontal_position
        Integer representing the final horizontal position.
    """

    aim = 0
    depth = 0
    horizontal_position = 0

    with open(filename, "r") as file:
        for line in file:
            input, value = line.split()
            value = int(value)
            if input == "forward":
                horizontal_position += value
                depth += value * aim
            elif input == "down":
                aim += value
            elif input == "up":
                aim -= value

    return depth, horizontal_position


if __name__ == "__main__":
    depth, horizontal_position = get_position_from_file(INPUT_FILENAME)
    print("Part1: Horizontal position times depth:", depth * horizontal_position)

    depth, horizontal_position = get_position_with_aim_from_file(INPUT_FILENAME)
    print("Part2: Horizontal position times depth:", depth * horizontal_position)
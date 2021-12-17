from typing import List, Tuple


def read_area_boundaries(filename: str) -> Tuple[Tuple[int, ...], Tuple[int, ...]]:
    with open(filename, "r") as file:
        line = file.read().strip()

    *_, x, y = line.split(" ")
    x = (x.replace("x=", "").replace(",", "")).split("..")
    y = (y.replace("y=", "")).split("..")
    x = tuple([int(boundary) for boundary in x])
    y = tuple([int(boundary) for boundary in y])

    return x, y


def triangular_number(n: int) -> int:
    return n * (n + 1) // 2


def find_max_y_position(lower_y_boundary: int) -> int:
    return triangular_number(lower_y_boundary)


def in_area(
    position: List[int],
    x_boundary: Tuple[int, int],
    y_boundary: Tuple[int, int],
) -> bool:
    x, y = position
    x_0, x_1 = x_boundary
    y_0, y_1 = y_boundary

    if x > x_1 or y < y_0:
        raise ValueError("the probe missed!")

    return (x_0 <= x <= x_1) and (y_0 <= y <= y_1)


def simulate_trajectory(
    velocity: List[int],
    x_boundary: Tuple[int, int],
    y_boundary: Tuple[int, int],
) -> List[int]:
    position = [0, 0]

    steps = 0
    while not in_area(position, x_boundary, y_boundary):
        steps += 1

        x, y = position
        x_vel, y_vel = velocity

        x += x_vel
        y += y_vel
        position = [x, y]

        y_vel -= 1
        if x_vel > 0:
            x_vel -= 1
        elif x_vel < 0:
            x_vel += 1
        velocity = [x_vel, y_vel]

    return position


def get_all_possible_initial_velocities(
    x_boundary: Tuple[int, int],
    y_boundary: Tuple[int, int],
) -> List[List[int]]:
    x_max = abs(max(x_boundary, key=lambda x: abs(x))) * 2
    y_max = abs(max(y_boundary, key=lambda y: abs(y))) * 2

    initial_velocities: List[List[int]] = []
    for x_vel in range(-x_max, x_max):
        for y_vel in range(-y_max, y_max):
            try:
                position = simulate_trajectory([x_vel, y_vel], x_boundary, y_boundary)
                initial_velocities.append(position)
            except ValueError:
                continue

    return initial_velocities


if __name__ == "__main__":
    INPUT_FILE = "input.txt"
    x_boundary, y_boundary = read_area_boundaries(filename=INPUT_FILE)
    print("Part 1:")
    y_max = find_max_y_position(y_boundary[0])
    print(f"The highest y position is {y_max}.")

    print("\nPart 2:")
    initial_velocities = get_all_possible_initial_velocities(x_boundary, y_boundary)
    print(f"There are {len(initial_velocities)} that satisfies the problem.")

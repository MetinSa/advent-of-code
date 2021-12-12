from typing import Dict, List, Set, Tuple


def read_cave_connections(filename: str) -> List[Tuple[str, str]]:
    pairs: List[Tuple[str, str]] = []
    with open(filename, "r") as file:
        for line in file:
            a, b = line.strip().split("-")
            pairs.append((a, b))

    return pairs


def get_caves(cave_connections: List[Tuple[str, str]]) -> Dict[str, Set[str]]:
    caves: Dict[str, Set[str]] = {}

    for connection in cave_connections:
        for cave in connection:
            if cave not in caves:
                caves[cave] = set()

    for connection in cave_connections:
        cave1, cave2 = connection
        caves[cave1].add(cave2)
        caves[cave2].add(cave1)

    return caves


def get_full_paths(caves: Dict[str, Set[str]]) -> List[List[str]]:

    all_paths: List[List[str]] = [["start"]]
    full_paths: List[List[str]] = []

    for path in all_paths:
        connections = caves[path[-1]]
        for connection in connections:
            if connection.islower() and connection in path:
                continue
            if connection == "start":
                continue
            if connection == "end":
                full_paths.append(path + [connection])
            all_paths.append(path + [connection])

    return sorted(full_paths)


def get_full_paths_with_new_rules(caves: Dict[str, Set[str]]) -> List[List[str]]:
    all_paths: List[List[str]] = [["not-visited", "start"]]
    full_paths: List[List[str]] = []

    for path in all_paths:
        connections = caves[path[-1]]
        for connection in connections:
            if connection.islower() and connection in path:
                if path[0] == "visited":
                    continue
                else:
                    if connection not in ("start", "stop"):
                        all_paths.append(["visited"] + path[1:] + [connection])
                    continue
            if connection == "start":
                continue
            if connection == "end":
                full_paths.append(path + [connection])
            all_paths.append(path + [connection])

    return sorted(full_paths)


if __name__ == "__main__":
    INPUT_FILE = "input.txt"

    connections = read_cave_connections(filename=INPUT_FILE)
    caves = get_caves(cave_connections=connections)

    print("Part 1:")
    full_paths = get_full_paths(caves=caves)
    print(f"There are {len(full_paths)} paths that visit small caves at most once.")

    print("\nPart 2:")
    full_paths = get_full_paths_with_new_rules(caves=caves)
    print(f"There are {len(full_paths)} paths with the new rules.")

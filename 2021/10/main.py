from typing import List, Optional


MAPPINGS = {"(": ")", "{": "}", "[": "]", "<": ">"}
CLOSINGS = list(MAPPINGS.values())

CORRUPT_POINTS = {")": 3, "]": 57, "}": 1197, ">": 25137}
COMPLETION_POINTS = {")": 1, "]": 2, "}": 3, ">": 4}


def read_input(filename: str) -> List[str]:
    lines: List[str] = []
    with open(filename, "r") as file:
        for line in file:
            lines.append(line.strip())

    return lines


def line_has_closing(characters: List[str]) -> bool:
    return any(char in CLOSINGS for char in characters)


def find_first_corrupt_char(line: str, verbose: bool = False) -> Optional[str]:
    characters = list(line)
    while line_has_closing(characters):
        for idx, character in enumerate(characters):
            if character in CLOSINGS:
                mapping = MAPPINGS[characters[idx - 1]]
                if mapping == character:
                    characters.pop(idx)
                    characters.pop(idx - 1)
                    break
                else:
                    if verbose:
                        print(f"Expected {mapping}, but found {character} instead.")
                    return character


def get_all_first_corrupt_chars(lines: List[str], verbose: bool = False) -> List[str]:
    first_corrupt_chars: List[str] = []
    for line in lines:
        corrupt_char = find_first_corrupt_char(line, verbose=verbose)
        if corrupt_char is not None:
            first_corrupt_chars.append(corrupt_char)

    return first_corrupt_chars


def filter_out_corrupt_lines(lines: List[str]) -> List[str]:
    non_currupt_lines: List[str] = []
    for line in lines:
        if find_first_corrupt_char(line) is None:
            non_currupt_lines.append(line)

    return non_currupt_lines


def get_completion(line: str) -> str:
    characters = list(line)
    while line_has_closing(characters):
        for idx, character in enumerate(characters):
            if character in CLOSINGS:
                mapping = MAPPINGS[characters[idx - 1]]
                if mapping == character:
                    characters.pop(idx)
                    characters.pop(idx - 1)
                    break

    return "".join([MAPPINGS[char] for char in reversed(characters)])


def get_completion_scores(completions: List[str]) -> List[int]:
    scores: List[int] = []
    for completion in completions:
        score = 0
        for character in completion:
            score *= 5
            score += COMPLETION_POINTS[character]
        scores.append(score)

    return scores


def get_middle_score(scores: List[int]) -> int:
    scores = sorted(scores)

    return scores[int(len(scores) / 2)]


if __name__ == "__main__":
    INPUT_FILE = "input.txt"
    lines = read_input(filename=INPUT_FILE)

    print("Part 1:")
    first_corrupt_chars = get_all_first_corrupt_chars(lines, verbose=False)
    points = sum(CORRUPT_POINTS[char] for char in first_corrupt_chars)
    print(f"The total syntax error score is {points}")

    print("\nPart 2:")
    non_corrupt_lines = filter_out_corrupt_lines(lines)
    completions: List[str] = []
    for line in non_corrupt_lines:
        completions.append(get_completion(line))
    scores = get_completion_scores(completions)
    middle_score = get_middle_score(scores)
    print(f"The middle completion score is {middle_score}")

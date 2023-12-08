from typing import Optional


def find_normalized_digit_in_window(substr: str) -> Optional[int]:
    DIGITS = {'one': 1, 'two': 2, 'three': 3, 'four': 4,
              'five': 5, 'six': 6, 'seven': 7, 'eight': 8, 'nine': 9}

    for c in substr:
        if c.isnumeric():
            return int(c)

    for k, v in DIGITS.items():
        if substr.find(k) != -1:
            return v

    return None


def concat_first_and_last_digit(line: str) -> int:
    first_digit = -99999
    last_digit = -99999
    for i in range(0, len(line)+1):
        first_digit = find_normalized_digit_in_window(line[0:i])
        if first_digit is not None:
            break

    for i in range(len(line)-1, -1, -1):
        print(line[i:len(line)])
        last_digit = find_normalized_digit_in_window(line[i:len(line)])
        if last_digit is not None:
            break

    return int(f'{first_digit}{last_digit}')


def part1(data: list[str]) -> int:
    numbers = [concat_first_and_last_digit(line) for line in data]
    answer = sum(numbers)
    return answer


def part2(data: list[str]) -> int:
    numbers = [concat_first_and_last_digit(line) for line in data]
    print(numbers)
    answer = sum(numbers)
    return answer


def main():
    with open('input/day1.txt') as f:
        data = [line for line in f.read().split() if line != '']

    print(part2(data))


if __name__ == '__main__':
    main()

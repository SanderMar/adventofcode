#!/usr/bin/env python

import sys
import requests as req
import datetime

INPUT_DIR = "input"


def read_token(file=".token"):
    with open(file, "r") as f:
        return f.read().strip()


def request_input(day: int, year: int, token_file: str | None = None):
    url = f"https://adventofcode.com/{year}/day/{day}/input"
    if token_file is None:
        token = read_token()
    else:
        token = read_token(token_file)

    cookies = {"session": token}

    resp = req.get(url, cookies=cookies)
    if resp.status_code != 200:
        print(f"Failed to get input for day {day} year {year}")
        print(f"Status code: {resp.status_code}")
        print(resp.text)
        sys.exit(1)
    return resp.text


def save_input(day: int, input: str, dir: str):
    day_num_str = str(day).zfill(2)
    with open(f"{dir}/day{day_num_str}.txt", "w") as f:
        f.write(input)


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: input.py <day> <year>")
        sys.exit(1)

    input_loc = INPUT_DIR

    day = int(sys.argv[1])
    year = datetime.date.today().year
    if len(sys.argv) >= 3:
        year = int(sys.argv[2])
    if len(sys.argv) >= 4:
        input_loc = sys.argv[3]
    if len(sys.argv) >= 5:
        token_file = sys.argv[4]
    else:
        token_file = None

    input = request_input(day, year, token_file)
    save_input(day, input, input_loc)

from typing import List
from random import choice
from sys import argv

def get_file(filename: str) -> str:
    with open(filename, "r") as f:
        return f.read()
def find_todo(data: str) -> List[str]:
    result = list()
    past_todo_label = False
    for item in data.splitlines():
        if past_todo_label:
            result.append(item)
        if item == "# todo":
            past_todo_label = True
    return result

def remove_dash(data: List[str]) -> List[str]:
    return [i[2:] for i in data]

def get_name(data: str) -> str:
    end = 0
    for i, c in enumerate(data):
        if c == "]":
            end = i
            break
    return data[1:end]


def pop_out_of_barces(data: List[str]) -> List[str]:
    return [get_name(i) if i[0] == "[" else i for i in data]

def get_list():
    data = get_file("./README.md")
    return pop_out_of_barces(remove_dash(find_todo(data)))

def main():
    if len(argv) > 1 and argv[1] == "all":
        for item in get_list():
            print(item)
    else:
        langs = get_list()
        lang = choice(langs)
        print(lang)

if __name__ == "__main__":
    main()

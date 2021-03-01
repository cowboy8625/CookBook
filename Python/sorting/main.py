from time import sleep
from os import name, system


def clear():
    system("cls" if name == "nt" else "clear")


def rotated(array_2d):
    list_of_tuples = zip(*array_2d[::-1])
    return [list(elem) for elem in list_of_tuples]


def display(data):
    data.reverse()
    large = max(data)
    amount = len(data)
    array = [[" " for x in range(amount)] for y in range(large)]
    for idx, item in enumerate(data):
        for y in range(item):
            array[y][idx] = chr(9608)
    for line in rotated(rotated(array)):
        print(*line)


def swap(data, pos1, pos2):
    data[pos1], data[pos2] = data[pos2], data[pos1]
    return data


def sort(data):
    i = 1
    while i < len(data):
        j = i
        while j > 0 and data[j - 1] > data[j]:
            swap(data, j, j - 1)
            j -= 1
        i += 1
    return data


data = [5, 6, 7, 4, 1, 7, 10]

clear()
display(data)
data = sort(data)
display(data)

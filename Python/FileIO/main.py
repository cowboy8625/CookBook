"""
Resources:
    https://www.tutorialspoint.com/python3/python_files_io.htm
This is how you pull data from a file.
"""

filename = "text.txt"

with open(filename) as f:
    data = f.read()


print(data)


"""
To save data and will over write data in file
"""

with open(filename, "a") as f:
    f.writelines("This is new data from python script\n")

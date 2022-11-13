def print_message(func):
    def wrapper(*args, **kwargs):
        print("some adding was done")
        return func(*args, **kwargs)
    return wrapper

def print_with_msg(msg):
    def print_log(func):
        def wrapper(*args):
            print(msg)
            print(f"The sum of {args} is {sum(args)}")
            return func(*args)
        return wrapper
    return print_log

# @print_with_msg("Hey")
def add(x, y):
    return x + y

def t(n):
    [print(("*"*i).center(n*2)) for i in range(1, n*2,2)]

# print(add(1,2))
print(add(1,2))
print(print_with_msg("Custom")(add)(1,2))


def print_message(func):
    def wrapper(*args, **kwargs):
        print("some adding was done")
        return func(*args, **kwargs)
    return wrapper

def print_log(func):
    def wrapper(*args):
        print(f"The sum of {args} is {sum(args)}")
        return func(*args)
    return wrapper

def t(n):
    [print(("*"*i).center(n*2)) for i in range(1, n*2,2)]

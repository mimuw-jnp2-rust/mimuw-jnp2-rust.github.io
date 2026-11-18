from typing import Union

def print_s(s: Union[str, int, float]):
    match s:
        case str(c):
            print(c)
        case int(i):
            print(i)
        case float(d):
            print(d)
        case _:
            # Bonus question: how to change the code
            # so that the Python type checkers (e.g. `mypy`)
            # will complain when we'll extend the `s` type
            # without adding a new `case`?
            raise ValueError("Unsupported type")

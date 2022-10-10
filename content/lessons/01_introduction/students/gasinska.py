# sample 1 - different ways of removing elements from the list while iterating
list1 = [1, 2, 3, 4]
for idx, item in enumerate(list1):
    del item
list1

# [1, 2, 3, 4]

list2 = [1, 2, 3, 4]
for idx, item in enumerate(list2):
    list2.remove(item)
list2

# [2, 4]

list3 = [1, 2, 3, 4]
for idx, item in enumerate(list3[:]):
    list3.remove(item)
list3

# []

list4 = [1, 2, 3, 4]
for idx, item in enumerate(list4):
    list4.pop(idx)
list4

# [2, 4]

# sample 2 - string interning
a = "abc"
b = "abc"
a is b

# True

a = ''.join(['a', 'b', 'c'])
b = ''.join(['a', 'b', 'c'])
a is b

# False

a = "abc!"
b = "abc!"
a is b

# False

# sample 3 - chained operations
(False == False) in [False]

# False

False == (False in [False])

# False

False == False in [False] # unexpected...

# True

# sample 4 - is operator
a = 256
b = 256
a is b

# True

a = 257
b = 257
a is b

# False

a, b = 257, 257
a is b

# True

257 is 257

# <>:1: SyntaxWarning: "is" with a literal. Did you mean "=="?
# <>:1: SyntaxWarning: "is" with a literal. Did you mean "=="?
# C:\Users\kgasinsk\AppData\Local\Temp\ipykernel_15776\331119389.py:1: SyntaxWarning: "is" with a literal. Did you mean "=="?
#  257 is 257

# sample 5 - local variables
def f(trufel):
    if trufel:
        y = 1
    y += 1

f(True) # everything is fine

f(False) # gives error: local variable 'y' referenced before assignment

# ---------------------------------------------------------------------------
# UnboundLocalError                         Traceback (most recent call last)
# Input In [17], in <cell line: 1>()
# ----> 1 f(False)

# Input In [15], in f(trufel)
#       3 if trufel:
#       4     y = 1
# ----> 5 y += 1

# UnboundLocalError: local variable 'y' referenced before assignment
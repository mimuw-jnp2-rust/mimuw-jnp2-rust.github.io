def add_contents(input_list, contents=[]):
     for val in input_list:
         contents.append(val)
     return contents

print(add_contents([1])) # [1]
print(add_contents([2])) # [1, 2]
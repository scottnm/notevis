"""
ideas:
    - allow different tuninings
    - allow different string counts
    - allow different fret "views"
    - pick unique colors for different notes highlighted
"""

def print_string(string, header = ""):
    print("{:3s}{:s}".format(header, string))

# print a single string (with a fret header)
#    | 01 | 02 | 03 | 04 |
# G. | -- | -- | -- | -- |

# print the header
fret_count = 17
header_row = "|"
for fret in range(1, fret_count + 1):
    header_row += " {:02d} |".format(fret)

string = "|"
for fret in range(1, fret_count + 1):
    string += " -- |"

print_string(header_row)
print_string(string, "G.")

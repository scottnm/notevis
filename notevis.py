"""
ideas:
    - allow different tuninings
    - allow different string counts
    - allow different fret "views"
    - pick unique colors for different notes highlighted
    - support sharps and flats
"""

def print_string(string, header = ""):
    print("{:3s}{:s}".format(header, string))

# print a single string (with a fret header)
#    | 01 | 02 | 03 | 04 |
# G. | -- | -- | -- | -- |

@enum.unique
class Note(enum.Enum):
    A  = 1
    AB = 2
    B  = 3
    C  = 4
    CD = 5
    D  = 6
    DE = 7
    E  = 8
    F  = 9
    FG = 10
    G  = 11

def generate_string(open_note):
    for fret in range(1, fret_count + 1):
    pass

# print the header
fret_count = 17
header_row = "|"
for fret in range(1, fret_count + 1):
    header_row += " {:02d} |".format(fret)

string = "|"
for fret in range(1, fret_count + 1):
    string += " -- |"

print_string(header_row)
for string_tuning in reversed(['E', 'A', 'D', 'G', 'B', 'E']):
    print_string(string, "{}.".format(string_tuning))


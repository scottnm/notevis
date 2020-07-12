fn print_string(header: &str, string: &str) {
    println!("{:3}{}", header, string);
}

fn print_string_key(key_string: &str) {
    print_string("", key_string);
}

fn main() {
    // print the header
    const FRET_COUNT: u8 = 17;
    let mut header_row = String::from("|");
    for fret in 1..FRET_COUNT + 1 {
        header_row = format!("{} {:02} |", header_row, fret);
    }

    let mut string = String::from("|");
    for _fret in 1..FRET_COUNT + 1 {
        string = format!("{} -- |", string);
    }

    print_string_key(&header_row);
    let strings = ['E', 'A', 'D', 'G', 'B', 'E'];
    for string_tuning in strings.iter().rev() {
        print_string(&format!("{}.", string_tuning), &string);
    }
}

/*
ideas:
    - allow different tuninings
    - allow different string counts
    - allow different fret "views"
    - pick unique colors for different notes highlighted
    - support sharps and flats
    */

/*
# print a single string (with a fret header)
#    | 01 | 02 | 03 | 04 |
# G. | -- | -- | -- | -- |
*/

/*
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
    for fret in range(1, FRET_COUNT + 1):
    pass
*/


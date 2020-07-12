fn print_string(fret_count: u8, header: &str) {
    let mut string = String::from("|");
    for _fret in 1..(fret_count + 1) {
        string = format!("{} -- |", string);
    }

    println!("{:3}{}", header, string);
}

fn print_string_key(fret_count: u8) {
    let mut header_row = String::from("|");
    for fret in 1..(fret_count + 1) {
        header_row = format!("{} {:02} |", header_row, fret);
    }
    println!("{:3}{}", "", header_row);
}

fn main() {
    // print the header
    const FRET_COUNT: u8 = 17;
    print_string_key(FRET_COUNT);

    let strings = ['E', 'A', 'D', 'G', 'B', 'E'];
    for string_tuning in strings.iter().rev() {
        print_string(FRET_COUNT, &format!("{}.", string_tuning));
    }
}

/*
ideas:
    - allow different tuninings
    - allow different string counts
    - allow different fret "views"
    - pick unique colors for different notes highlighted
    - support sharps and flats
    - highlight the numbers in the header row for the dots
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


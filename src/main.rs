#[derive(Clone, Copy)]
enum Note {
    A,
    AB,
    B,
    C,
    CD,
    D,
    DE,
    E,
    F,
    FG,
    G,
    GA,
}

impl Note {
    fn render(&self) -> &str {
        match self {
            Note::A => "A",
            Note::AB => "AB",
            Note::B => "B",
            Note::C => "C",
            Note::CD => "CD",
            Note::D => "D",
            Note::DE => "DE",
            Note::E => "E",
            Note::F => "F",
            Note::FG => "FG",
            Note::G => "G",
            Note::GA => "GA",
        }
    }

    fn next(&self) -> Self {
        match self {
            Note::A => Note::AB,
            Note::AB => Note::B,
            Note::B => Note::C,
            Note::C => Note::CD,
            Note::CD => Note::D,
            Note::D => Note::DE,
            Note::DE => Note::E,
            Note::E => Note::F,
            Note::F => Note::FG,
            Note::FG => Note::G,
            Note::G => Note::GA,
            Note::GA => Note::A,
        }
    }
}

fn print_string(fret_count: u8, string_tuning: Note) {
    let header = format!("{:2}.", string_tuning.render());

    let mut string = String::from("|");

    let mut note = string_tuning;
    for _fret in 1..(fret_count + 1) {
        note = note.next();
        string = format!("{} {:2} |", string, note.render());
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

    let strings = [Note::E, Note::A, Note::D, Note::G, Note::B, Note::E];
    for string_tuning in strings.iter().rev() {
        print_string(FRET_COUNT, *string_tuning);
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


/*
ideas:
    - allow different tuninings
    - allow different string counts
    - allow different fret "views"
    - pick unique colors for different notes highlighted
    - support sharps and flats
    - highlight the numbers in the header row for the dots
    */

#[derive(Clone, Copy, PartialEq)]
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

impl From<&str> for Note {
    fn from(s: &str) -> Self {
        const MAPPINGS: [(&str, Note); 17] = [
            ("a", Note::A),
            ("a#", Note::AB),
            ("bb", Note::AB),
            ("b", Note::B),
            ("c", Note::C),
            ("c#", Note::CD),
            ("db", Note::CD),
            ("d", Note::D),
            ("d#", Note::DE),
            ("eb", Note::DE),
            ("e", Note::E),
            ("f", Note::F),
            ("f#", Note::FG),
            ("gb", Note::FG),
            ("g", Note::G),
            ("g#", Note::GA),
            ("ab", Note::GA),
        ];

        for (note_string, note) in &MAPPINGS {
            if note_string.eq_ignore_ascii_case(s) {
                return *note;
            }
        }

        panic!("Could not parse string as note! [{}]", s);
    }
}

fn print_string(fret_count: u8, string_tuning: Note, notes_to_show: &[Note]) {
    let mut string = String::from("|");

    let mut note = string_tuning;
    for _fret in 1..(fret_count + 1) {
        note = note.next();

        let fret_value = if notes_to_show.contains(&note) {
            note.render()
        } else {
            "--"
        };

        string = format!("{} {:2} |", string, fret_value);
    }

    let header = format!("{}.", string_tuning.render());
    println!("{:3}{}", header, string);
}

fn print_legend(fret_count: u8) {
    let mut legend = String::from("|");
    for fret in 1..(fret_count + 1) {
        legend = format!("{} {:02} |", legend, fret);
    }
    println!("{:3}{}", "", legend);
}

fn main() {
    let args = std::env::args();
    let note_args = args.skip(1);
    let notes: Vec<Note> = note_args.map(|note_arg| Note::from(&note_arg[..])).collect();

    const FRET_COUNT: u8 = 17;
    print_legend(FRET_COUNT);

    let strings = [Note::E, Note::A, Note::D, Note::G, Note::B, Note::E];
    for string_tuning in strings.iter().rev() {
        print_string(FRET_COUNT, *string_tuning, notes.as_slice());
    }
}

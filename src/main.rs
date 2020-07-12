// General improvements/TODOs
// - allow different tuninings
// - allow different string counts
// - allow different fret ranges besides 1->17
// - pick unique colors for different notes highlighted

use ansi_term::Colour::Black as ATBlack;
use ansi_term::Colour::White as ATWhite;

#[derive(Clone, Copy, PartialEq)]
enum Note {
    A,
    As,
    Bb,
    B,
    C,
    Cs,
    Db,
    D,
    Ds,
    Eb,
    E,
    F,
    Fs,
    Gb,
    G,
    Gs,
    Ab,
}

impl Note {
    fn render(&self) -> &str {
        match self {
            Note::A => "A",
            Note::As => "A#",
            Note::Bb => "Bb",
            Note::B => "B",
            Note::C => "C",
            Note::Cs => "C#",
            Note::Db => "Db",
            Note::D => "D",
            Note::Ds => "D#",
            Note::Eb => "Eb",
            Note::E => "E",
            Note::F => "F",
            Note::Fs => "F#",
            Note::Gb => "Gb",
            Note::G => "G",
            Note::Gs => "G#",
            Note::Ab => "Ab",
        }
    }

    fn next(&self, key: &[Note]) -> Self {
        match self {
            Note::A => if key.contains(&Note::As) { Note::As } else { Note::Bb },
            Note::As => Note::B,
            Note::Bb => Note::B,
            Note::B => Note::C,
            Note::C => if key.contains(&Note::Cs) { Note::Cs } else { Note::Db },
            Note::Cs => Note::D,
            Note::Db => Note::D,
            Note::D => if key.contains(&Note::Ds) { Note::Ds } else { Note::Eb },
            Note::Ds => Note::E,
            Note::Eb => Note::E,
            Note::E => Note::F,
            Note::F => if key.contains(&Note::Fs) { Note::Fs } else { Note::Gb },
            Note::Fs => Note::G,
            Note::Gb => Note::G,
            Note::G => if key.contains(&Note::Gs) { Note::Gs } else { Note::Ab },
            Note::Gs => Note::A,
            Note::Ab => Note::A,
        }
    }
}

impl From<&str> for Note {
    fn from(s: &str) -> Self {
        const MAPPINGS: [(&str, Note); 17] = [
            ("a", Note::A),
            ("a#", Note::As),
            ("bb", Note::Bb),
            ("b", Note::B),
            ("c", Note::C),
            ("c#", Note::Cs),
            ("db", Note::Db),
            ("d", Note::D),
            ("d#", Note::Ds),
            ("eb", Note::Eb),
            ("e", Note::E),
            ("f", Note::F),
            ("f#", Note::Fs),
            ("gb", Note::Gb),
            ("g", Note::G),
            ("g#", Note::Gs),
            ("ab", Note::Ab),
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
        // TODO: currently passing notes_to_show as the 'key' which is good enough since we don't
        // do anything with the sharps/flats not expliclty specified but it could lead to odd
        // behavior
        note = note.next(notes_to_show);

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
    const INLAYS: [u8; 10] = [3, 5, 7, 9, 12, 15, 17, 19, 21, 24];

    let mut legend = String::from("|");
    for fret in 1..(fret_count + 1) {
        let fret_str = format!("{:02}", fret);
        let fret_str = if INLAYS.contains(&fret) {
            ATBlack.on(ATWhite).paint(fret_str)
        } else {
            ATWhite.on(ATBlack).paint(fret_str)
        };

        legend = format!("{} {} |", legend, fret_str);
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

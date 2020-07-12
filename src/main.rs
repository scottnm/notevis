#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn colour(&self) -> ansi_term::Colour {
        use ansi_term::Colour;
        match self {
            Note::A => Colour::Fixed(7),
            Note::As => Colour::Fixed(15),
            Note::Bb => Colour::Fixed(15),
            Note::B => Colour::Fixed(1),
            Note::C => Colour::Fixed(2),
            Note::Cs => Colour::Fixed(10),
            Note::Db => Colour::Fixed(10),
            Note::D => Colour::Fixed(3),
            Note::Ds => Colour::Fixed(11),
            Note::Eb => Colour::Fixed(11),
            Note::E => Colour::Fixed(4),
            Note::F => Colour::Fixed(5),
            Note::Fs => Colour::Fixed(13),
            Note::Gb => Colour::Fixed(13),
            Note::G => Colour::Fixed(6),
            Note::Gs => Colour::Fixed(14),
            Note::Ab => Colour::Fixed(14),
        }
    }

    fn next(&self, key: &[Note]) -> Self {
        match self {
            Note::A => {
                if key.contains(&Note::As) {
                    Note::As
                } else {
                    Note::Bb
                }
            }
            Note::As => Note::B,
            Note::Bb => Note::B,
            Note::B => Note::C,
            Note::C => {
                if key.contains(&Note::Cs) {
                    Note::Cs
                } else {
                    Note::Db
                }
            }
            Note::Cs => Note::D,
            Note::Db => Note::D,
            Note::D => {
                if key.contains(&Note::Ds) {
                    Note::Ds
                } else {
                    Note::Eb
                }
            }
            Note::Ds => Note::E,
            Note::Eb => Note::E,
            Note::E => Note::F,
            Note::F => {
                if key.contains(&Note::Fs) {
                    Note::Fs
                } else {
                    Note::Gb
                }
            }
            Note::Fs => Note::G,
            Note::Gb => Note::G,
            Note::G => {
                if key.contains(&Note::Gs) {
                    Note::Gs
                } else {
                    Note::Ab
                }
            }
            Note::Gs => Note::A,
            Note::Ab => Note::A,
        }
    }
}

impl std::str::FromStr for Note {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
                return Ok(*note);
            }
        }

        Err(format!("Could not parse '{}' as note!", s))
    }
}

fn color_inlay(fret: u8, fret_str: &str) -> ansi_term::ANSIString {
    use ansi_term::Colour::Black as ATBlack;
    use ansi_term::Colour::White as ATWhite;

    const INLAYS: [u8; 10] = [3, 5, 7, 9, 12, 15, 17, 19, 21, 24];
    if INLAYS.contains(&fret) {
        ATBlack.on(ATWhite).paint(fret_str)
    } else {
        ATWhite.on(ATBlack).paint(fret_str)
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

        let (fret_value, fret_colour) = if notes_to_show.contains(&note) {
            (note.render(), note.colour())
        } else {
            ("--", ansi_term::Colour::White)
        };

        let fret_value = format!("{:2}", fret_value);
        string = format!("{} {} |", string, fret_colour.paint(fret_value));
    }

    let (string_header, string_header_colour) = if notes_to_show.contains(&string_tuning) {
        (string_tuning.render(), string_tuning.colour())
    } else {
        (string_tuning.render(), ansi_term::Colour::White)
    };

    println!("{} {}", string_header_colour.paint(string_header), string);
}

fn print_legend(fret_count: u8) {
    let mut legend = String::from("|");
    for fret in 1..(fret_count + 1) {
        let fret_str = format!("{:02}", fret);
        legend = format!("{} {} |", legend, color_inlay(fret, &fret_str));
    }
    println!("{:2}{}", "", legend);
}

use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone, Copy)]
#[structopt()]
enum Tuning {
    Standard,
    DropD,
}

impl std::str::FromStr for Tuning {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const MAPPINGS: [(&str, Tuning); 2] =
            [("standard", Tuning::Standard), ("dropd", Tuning::DropD)];

        for (tuning_string, tuning) in &MAPPINGS {
            if tuning_string.eq_ignore_ascii_case(s) {
                return Ok(*tuning);
            }
        }

        Err(format!("Failed to parse '{}' as tuning", s))
    }
}

impl Tuning {
    fn as_strings(&self) -> [Note; 6] {
        match self {
            Tuning::Standard => [Note::E, Note::A, Note::D, Note::G, Note::B, Note::E],
            Tuning::DropD => [Note::D, Note::A, Note::D, Note::G, Note::B, Note::E],
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt()]
struct Options {
    #[structopt(short, long)]
    notes: Vec<Note>,

    #[structopt(short, long, default_value = "Standard")]
    tuning: Tuning,
}

fn main() {
    let options = Options::from_args();

    let strings = options.tuning.as_strings();

    const FRET_COUNT: u8 = 17;
    print_legend(FRET_COUNT);
    println!();

    for string_tuning in strings.iter().rev() {
        print_string(FRET_COUNT, *string_tuning, options.notes.as_slice());
    }

    println!();
    println!("Tuning: {:?}", options.tuning);
}

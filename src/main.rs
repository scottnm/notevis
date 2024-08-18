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
            Note::A => "A ",
            Note::As => "A#",
            Note::Bb => "Bb",
            Note::B => "B ",
            Note::C => "C ",
            Note::Cs => "C#",
            Note::Db => "Db",
            Note::D => "D ",
            Note::Ds => "D#",
            Note::Eb => "Eb",
            Note::E => "E ",
            Note::F => "F ",
            Note::Fs => "F#",
            Note::Gb => "Gb",
            Note::G => "G ",
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

fn print_empty_string(fret_count: u8, string_tuning: Note) {
    let mut string = String::from("|");

    for _fret in 1..(fret_count + 1) {
        string = format!("{} -- |", string);
    }

    let string_header = string_tuning.render();
    let string_header_colour = ansi_term::Colour::White;
    println!("{} {}", string_header_colour.paint(string_header), string);
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

        string = format!("{} {} |", string, fret_colour.paint(fret_value));
    }

    let string_header = string_tuning.render();
    let string_header_colour = if notes_to_show.contains(&string_tuning) {
        string_tuning.colour()
    } else {
        ansi_term::Colour::White
    };

    println!("{} {}", string_header_colour.paint(string_header), string);
}

fn print_fret_tab(fret_count: u8, string_tuning: Note, opt_fret_to_show: Option<u8>) {
    if let None = opt_fret_to_show {
        print_empty_string(fret_count, string_tuning);
        return;
    }

    let fret_to_show = opt_fret_to_show.unwrap();

    let mut string = String::from("|");
    let mut note = string_tuning;
    for fret in 1..(fret_count + 1) {
        // TODO: need a key so we can properly determine whether the next note is a sharp or flat
        note = note.next(&[]);

        let (fret_value, fret_colour) = if fret == fret_to_show {
            (note.render(), note.colour())
        } else {
            ("--", ansi_term::Colour::White)
        };

        string = format!("{} {} |", string, fret_colour.paint(fret_value));
    }

    let string_header = string_tuning.render();
    let string_header_colour = if fret_to_show == 0 {
        string_tuning.colour()
    } else {
        ansi_term::Colour::White
    };

    println!("{} {}", string_header_colour.paint(string_header), string);
}

fn print_legend(fret_count: u8) {
    let mut legend = String::from("|");
    for fret in 1..(fret_count + 1) {
        let fret_str = format!("{:02}", fret);
        legend = format!("{} {} |", legend, color_inlay(fret, &fret_str));
    }
    println!("{:3}{}", "", legend);
}

use structopt::StructOpt;

#[derive(Debug)]
struct ChordTab {
    frets: [Option<u8>; 6],
}

impl std::str::FromStr for ChordTab {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ct = ChordTab { frets: [None; 6] };
        let input_frets: Vec<&str> = s.split(' ').collect();

        if ct.frets.len() != input_frets.len() {
            return Err(format!(
                "Expected {} strings but found {}!",
                ct.frets.len(),
                input_frets.len()
            ));
        }

        for (fret, fretted_string) in ct.frets.iter_mut().zip(input_frets.iter()) {
            if fretted_string.eq_ignore_ascii_case("x") {
                *fret = None;
            } else {
                if let Ok(parsed_fretted_string) = fretted_string.parse::<u8>() {
                    *fret = Some(parsed_fretted_string);
                    continue;
                }

                return Err(format!("'{}' couldn't be parsed as a fret", fretted_string));
            }
        }

        Ok(ct)
    }
}

#[derive(Debug, StructOpt, Clone, Copy)]
#[structopt()]
enum Tuning {
    Standard,
    DropD,
    DropC,
    DropB,
}

impl std::str::FromStr for Tuning {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const MAPPINGS: [(&str, Tuning); 4] = [
            ("standard", Tuning::Standard),
            ("dropd", Tuning::DropD),
            ("dropc", Tuning::DropC),
            ("dropb", Tuning::DropB),
        ];

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
            Tuning::DropC => [Note::C, Note::G, Note::C, Note::F, Note::A, Note::D],
            Tuning::DropB => [Note::B, Note::Fs, Note::B, Note::E, Note::Gs, Note::Cs],
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt()]
struct Options {
    #[structopt(short, long, help = "e.g. C D Eb F G Ab B")]
    notes: Option<Vec<Note>>,

    #[structopt(short, long, default_value = "Standard")]
    tuning: Tuning,

    #[structopt(short, long, help = "e.g. 'x x 0 2 2 3'")]
    chord_tab: Option<ChordTab>,

    #[structopt(short, long, default_value = "17")]
    fret_count: u8,
}

fn main() {
    let options = Options::from_args();
    let strings = options.tuning.as_strings();

    if let Some(notes) = options.notes {
        print_legend(options.fret_count);
        println!();

        for string in strings.iter().rev() {
            print_string(options.fret_count, *string, notes.as_slice());
        }

        println!();
    }

    if let Some(chord_tab) = options.chord_tab {
        assert!(strings.len() == chord_tab.frets.len());

        print_legend(options.fret_count);
        println!();

        for (string, fret_tab) in strings.iter().rev().zip(chord_tab.frets.iter()) {
            print_fret_tab(options.fret_count, *string, *fret_tab)
        }

        println!();
    }

    println!("Tuning: {:?}", options.tuning);
}

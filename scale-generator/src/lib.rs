// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.

const SHARPS: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const FLATS: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

#[derive(Debug)]
pub struct Error(String);

#[derive(Debug)]
pub struct Scale {
    tonic: String,
    intervals: String,
    scales: [&'static str; 12],
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Self, Error> {
        if intervals
            .chars()
            .any(|ch| ch != 'm' && ch != 'M' && ch != 'A')
        {
            return Err(Error("invalid interval".to_string()));
        }
        match tonic {
            "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#" | "d#" | "C"
            | "a" => Ok(Self {
                tonic: normalize(tonic),
                intervals: intervals.to_string(),
                scales: SHARPS,
            }),
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => {
                Ok(Self {
                    tonic: normalize(tonic),
                    intervals: intervals.to_string(),
                    scales: FLATS,
                })
            }
            _ => Err(Error("invalid tonic".to_string())),
        }
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        let mut iter = self
            .scales
            .iter()
            .cycle()
            .skip_while(|&&s| s != self.tonic.as_str());
        let mut notes = vec![];
        notes.push(iter.next().unwrap().to_string());
        for i in self.intervals.chars() {
            match i {
                'm' => {
                    notes.push(iter.next().unwrap().to_string());
                }
                'M' => {
                    iter.next();
                    notes.push(iter.next().unwrap().to_string());
                }
                'A' => {
                    iter.next();
                    iter.next();
                    notes.push(iter.next().unwrap().to_string());
                }
                _ => unreachable!(),
            }
        }
        notes
    }
}

fn normalize(tonic: &str) -> String {
    tonic
        .chars()
        .enumerate()
        .map(|(i, ch)| if i == 0 { ch.to_ascii_uppercase() } else { ch })
        .collect()
}

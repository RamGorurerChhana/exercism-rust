use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};
/// Tournament to hold the points table
#[derive(Debug, Default)]
struct Tournament<'a> {
    points_table: HashMap<&'a str, TeamStats>,
}

impl<'a> Tournament<'a> {
    // add a match result for one particular line
    // depending on win/loss/draw update the points table
    // and TeamStats accordingly
    fn add_match_result(&mut self, input: &'a str) {
        let splitted = input.split(';').collect::<Vec<_>>();
        // each input line must have 3 segments team1;team2;match_result
        // in other input line including blank lines are ignored
        if splitted.len() < 3 {
            return;
        }
        match splitted[2] {
            "win" => {
                self.points_table.entry(splitted[0]).or_default().add_win();
                self.points_table.entry(splitted[1]).or_default().add_loss();
            }
            "loss" => {
                self.points_table.entry(splitted[0]).or_default().add_loss();
                self.points_table.entry(splitted[1]).or_default().add_win();
            }
            "draw" => {
                self.points_table.entry(splitted[0]).or_default().add_draw();
                self.points_table.entry(splitted[1]).or_default().add_draw();
            }
            _ => {}
        };
    }

    // parse the entire match_results string
    // split by newline character and add match result for each line
    fn parse_match_results(match_results: &'a str) -> Self {
        let mut t = Self::default();
        match_results
            .split('\n')
            .for_each(|s| t.add_match_result(s));
        t
    }
}

// implement Display trait for Tournament so that it can be converted to string
impl<'a> Display for Tournament<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // before converting to string the points_table need to be sorted on points
        // in case same points sort by team name alphabetically
        let mut v = self.points_table.iter().collect::<Vec<_>>();
        v.sort_unstable_by(|a, b| match a.1.points.partial_cmp(&b.1.points).unwrap() {
            Ordering::Equal => a.0.partial_cmp(&b.0).unwrap(),
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        });
        // write the header
        write!(
            f,
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            "Team", "MP", "W", "D", "L", "P"
        )?;
        // for each team write the stats
        for pt in v {
            write!(f, "\n")?;
            write!(
                f,
                "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
                pt.0, pt.1.match_played, pt.1.wins, pt.1.draws, pt.1.losses, pt.1.points
            )?;
        }
        Ok(())
    }
}

/// TeamStats struct will hold different stats for a particular team
#[derive(Debug, Default)]
struct TeamStats {
    match_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

impl TeamStats {
    // register draw for a team and update stats accordingly
    fn add_draw(&mut self) {
        self.match_played += 1;
        self.draws += 1;
        self.points += 1;
    }
    // register win for a team and update stats accordingly
    fn add_win(&mut self) {
        self.match_played += 1;
        self.wins += 1;
        self.points += 3;
    }
    // register loss for a team and update stats accordingly
    fn add_loss(&mut self) {
        self.match_played += 1;
        self.losses += 1;
    }
}

pub fn tally(match_results: &str) -> String {
    Tournament::parse_match_results(match_results).to_string()
}

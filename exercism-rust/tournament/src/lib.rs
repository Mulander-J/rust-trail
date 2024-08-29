use std::collections::HashMap;

enum GameOutcome {
    Win,
    Loss,
    Draw,
}

impl From<&str> for GameOutcome {
    fn from(origin: &str) -> GameOutcome {
        match origin {
            "win" => GameOutcome::Win,
            "loss" => GameOutcome::Loss,
            "draw" => GameOutcome::Draw,
            _ => panic!(),
        }
    }
}
impl GameOutcome {
    fn reverse(&self) -> Self {
        match self {
            GameOutcome::Win => GameOutcome::Loss,
            GameOutcome::Loss => GameOutcome::Win,
            GameOutcome::Draw => GameOutcome::Draw,
        }
    }
}

struct TeamTable {
    name: String,
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

impl TeamTable {
    fn new(_name: &str) -> Self {
        Self {
            name: _name.to_string(),
            mp: 0,
            w: 0,
            d: 0,
            l: 0,
            p: 0,
        }
    }
    fn play(&mut self, ret: GameOutcome) {
        self.mp += 1;
        match ret {
            GameOutcome::Win => {
                self.w += 1;
                self.p += 3;
            }
            GameOutcome::Draw => {
                self.d += 1;
                self.p += 1;
            }
            GameOutcome::Loss => self.l += 1,
        }
    }
}

impl From<&TeamTable> for String {
    fn from(origin: &TeamTable) -> String {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            origin.name, origin.mp, origin.w, origin.d, origin.l, origin.p
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, TeamTable> = HashMap::new();

    match_results.lines().for_each(|s| {
        let parts: Vec<_> = s.trim().split(';').collect();
        let [p1, p2, ret] = [parts[0], parts[1], parts[2]];

        teams
            .entry(p1.to_string())
            .or_insert(TeamTable::new(p1))
            .play(GameOutcome::from(ret));

        teams
            .entry(p2.to_string())
            .or_insert(TeamTable::new(p2))
            .play(GameOutcome::from(ret).reverse());
    });

    write_tally(&teams)
}

fn write_tally(teams: &HashMap<String, TeamTable>) -> String {
    let mut vals: Vec<_> = teams.values().collect();
    vals.sort_by(|&x, &y| y.p.cmp(&x.p).then_with(|| x.name.cmp(&y.name)));
    let mut lines = vec![format!("{:30} | MP |  W |  D |  L |  P", "Team")];
    lines.extend(vals.iter().map(|&t| t.into()));
    lines.join("\n")
}

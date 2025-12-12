use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Identities {
    pub values: Vec<(String, String)>,
}

pub fn main() -> anyhow::Result<()> {
    let contents = fs::read_to_string("data.ron")?;
    let identities: Identities = ron::from_str(&contents)?;

    let mut experiences: Vec<String> = identities
        .values
        .iter()
        .map(|(exp, _)| exp.to_string())
        .collect();

    let mut strangers: Vec<String> = identities
        .values
        .iter()
        .map(|(_, stranger)| stranger.to_string())
        .collect();

    experiences.sort();
    strangers.sort();

    let mut out = String::new();
    out.push_str("[table]\n".into());
    out.push_str("[row][header]Strangers[/header][header]Identities[/header][/row]".into());
    for i in 0..experiences.len() {
        out.push_str(&format!(
            "[row][cell]{}[/cell][cell]{}[/cell][/row]\n",
            strangers[i], experiences[i]
        ));
    }
    out.push_str("[/table]");

    fs::write("output.txt", out)?;

    Ok(())
}

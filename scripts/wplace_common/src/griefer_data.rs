pub struct GrieferData {
    username: String,
    user_id: u32,
    alliance_name: Option<String>,
    alliance_id: Option<u32>,
    description: Option<String>,
    alt_group: Option<String>,
}

const NO_DATA: &str = "-";

impl GrieferData {
    pub fn to_markdown_list(&self) -> String {
        format!(
            "- {username} #{user_id} {alliance_data}\n{description}{alt_group}",
            username = self.username,
            user_id = self.user_id,
            alliance_data = match &self.alliance_name {
                Some(alliance_name) => format!(
                    "[{alliance_name}] {alliance_id}",
                    alliance_id = match self.alliance_id {
                        Some(alliance_id) => format!("({alliance_id})"),
                        None => String::new(),
                    }
                ),
                None => String::new(),
            },
            description = match &self.description {
                Some(description) => format!("  * {description}\n"),
                None => String::new(),
            },
            alt_group = match &self.alt_group {
                Some(alt_group) => format!("  * Gruppo Alt: {alt_group}\n"),
                None => String::new(),
            }
        )
    }

    pub fn new() -> Vec<Self> {
        let data = super::GRIEFER_FILE;
        let mut lines = data.split('\n');
        let mut out: Vec<Self> = Vec::new();
        while let Some(_) = lines.next() {
            let username = lines.next().expect("Wrong line number").to_string();
            let user_id = lines
                .next()
                .expect("Wrong line number")
                .parse::<u32>()
                .expect("Error parsing User ID");
            let alliance_name = {
                let line = lines.next().expect("Wrong line number");
                match line == NO_DATA {
                    true => None,
                    false => Some(line.to_string()),
                }
            };
            let alliance_id = {
                let line = lines.next().expect("Wrong line number");
                match line == NO_DATA {
                    true => None,
                    false => Some(line.parse::<u32>().expect("Error parsing Alliance ID")),
                }
            };
            let description = {
                let line = lines.next().expect("Wrong line number");
                match line == NO_DATA {
                    true => None,
                    false => Some(line.to_string()),
                }
            };
            let alt_group = {
                let line = lines.next().expect("Wrong line number");
                match line == NO_DATA {
                    true => None,
                    false => Some(line.to_string()),
                }
            };

            for already_inserted in out.iter() {
                if already_inserted.user_id == user_id {
                    panic!(
                        "User {} is duplicate of {}",
                        already_inserted.username, username
                    );
                }

                if alliance_id.is_some()
                    && already_inserted.alliance_id == alliance_id
                    && already_inserted.alliance_name != alliance_name
                {
                    println!("Alliance ID {alliance_id:#?} matches under different names");
                }
            }

            out.push(GrieferData {
                username,
                user_id,
                alliance_name,
                alliance_id,
                description,
                alt_group,
            });
        }
        out
    }
}

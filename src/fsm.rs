use std::collections::{HashMap, HashSet};

pub struct Reaction<'a> {
    pub output: &'a str,
    pub target_state_name: &'a str,
}

pub struct Jump<'a> {
    pub state_name: &'a str,
    pub input: &'a str,
    pub reaction: Reaction<'a>,
}

pub struct Fsm<'a> {
    current_state: usize,
    names: Vec<&'a str>,
    jumps: Vec<HashMap<String, ReactionInternal>>,
}

struct ReactionInternal {
    output: String,
    target_state_id: usize,
}

impl<'a> Fsm<'a> {
    pub fn create(
        init_state: &str,
        states: &HashSet<&'a str>,
        jumps: impl IntoIterator<Item = &'a Jump<'a>>,
    ) -> Result<Self, &'a str> {
        let init_state_idx_opt = states.iter().position(|s| *s == init_state);

        let Some(init_state_idx_opt) = init_state_idx_opt else {
            return Err("Initial state is not present among states");
        };

        let mut jumps_processed: Vec<HashMap<String, ReactionInternal>> = Vec::new();
        for s in states {
            jumps_processed.push(HashMap::new());
        }

        for j in jumps {
            let idx_src = states.iter().position(|s| *s == j.state_name);
            let Some(idx_src) = idx_src else {
                return Err("Source state is not present among states");
            };
            let idx_target = states
                .iter()
                .position(|s| *s == j.reaction.target_state_name);
            let Some(idx_target) = idx_target else {
                return Err("Target state is not present among states");
            };
            jumps_processed[idx_src].insert(
                String::from(j.input),
                ReactionInternal {
                    output: String::from(j.reaction.output),
                    target_state_id: idx_target,
                },
            );
        }
        Ok(Fsm {
            current_state: init_state_idx_opt,
            names: states.iter().copied().collect(),
            jumps: jumps_processed,
        })
    }

    pub fn get_state(&self) -> &str {
        self.names[self.current_state]
    }

    pub fn input(&mut self, input_str: &str) -> Result<&str, String> {
        let reaction = &self.jumps[self.current_state].get(input_str);
        if let Some(reaction) = reaction {
            self.current_state = reaction.target_state_id;
            return Ok(reaction.output.as_str());
        }
        Err(format!(
            "No reaction found for state {} and input {}",
            self.current_state, input_str
        ))
    }
}


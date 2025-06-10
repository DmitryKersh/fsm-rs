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

pub struct Fsm {
    current_state: usize,
    names: Vec<String>,
    jumps: Vec<HashMap<String, ReactionInternal>>,
}

struct ReactionInternal {
    output: String,
    target_state_id: usize,
}

impl Fsm {
    pub fn create<'a>(init_state: &str, states: &HashSet<&str>, jumps: &Vec<Jump>) -> Result<Fsm, &'a str> {
        let init_state_idx_opt = states.iter().position(|s| *s == init_state);
        let names: Vec<String> = states.iter().map(|s| s.to_string()).collect();

        if init_state_idx_opt.is_none() {
            return Err("Initial state is not present among states");
        }

        let mut jumps_processed: Vec<HashMap<String, ReactionInternal>> = Vec::new();
        for s in states {
            jumps_processed.push(HashMap::new());
        }
        let mut idx_src: Option<usize>;
        let mut idx_target: Option<usize>;
        
        for j in jumps {
            idx_src = names.iter().position(|s| *s == j.state_name);
            if idx_src.is_none() {
                return Err("Source state is not present among states");
            }
            idx_target = names.iter().position(|s| *s == j.reaction.target_state_name);
            if idx_target.is_none() {
                return Err("Target state is not present among states");
            }
            jumps_processed[idx_src.unwrap()].insert(
                String::from(j.input), 
                ReactionInternal {
                    output: String::from(j.reaction.output), 
                    target_state_id: idx_target.unwrap()
                }
            );
        }
        Ok(Fsm {
            current_state: init_state_idx_opt.unwrap(),
            names,
            jumps: jumps_processed
        })
    }
    
    pub fn get_state(&self) -> & str {
        self.names[self.current_state].as_str()
    }
    
    pub fn input(&mut self, input_str: &str) -> Result<&str, String> {
        let reaction = &self.jumps[self.current_state].get(input_str);
        if reaction.is_some() { 
            self.current_state = reaction.unwrap().target_state_id;
            return Ok(reaction.unwrap().output.as_str());
        }
        Err(format!("No reaction found for state {} and input {}", self.current_state, input_str))
    }
}
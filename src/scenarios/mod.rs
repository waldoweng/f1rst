use clap;
use std::collections::hash_map;

use crate::testing::{RunFn, ScenarioFn};

pub struct Scenarios {
    Scenarios: hash_map::HashMap<String, Scenario>,
}

#[derive(Clone)]
pub struct Scenario {
    Name: String,
    Description: String,
    Parameters: Vec<ScenarioParameter>,
    ScenarioFn: ScenarioFn,
    RunFn: RunFn,
}

#[derive(Clone)]
pub struct ScenarioParameter {
    Name: String,
    Description: String,
    Default: String,
}

pub type ScenarioOption = fn(&mut Scenario);

fn parameter(parameter: ScenarioParameter) -> Box<dyn Fn(&mut Scenario)> {
    return Box::new(move |i: &mut Scenario| i.Parameters.push(parameter.clone()));
}

impl Scenarios {
    pub fn new() -> Self {
        Self {
            Scenarios: hash_map::HashMap::new(),
        }
    }

    pub fn add(&mut self, scenario: Scenario) {
        self.Scenarios.insert(scenario.Name.clone(), scenario);
    }

    pub fn get_scenario(&mut self, scenario_name: &str) -> Option<&Scenario> {
        self.Scenarios.get(scenario_name)
    }

    pub fn get_scenario_names(&self) -> Vec<&str> {
        let mut v = vec![];
        for s in self.Scenarios.keys() {
            v.push(s.as_ref())
        }
        return v;
    }
}

impl Scenarios {
    pub fn cmd(&self) -> clap::Command {
        clap::Command::new("scenario")
            .about("Prints information about available test scenarios")
            .subcommand(self.ls_cmd())
    }

    fn ls_cmd(&self) -> clap::Command {
        clap::Command::new("ls")
    }

    pub fn run(&self, cmd: clap::Command) {
        let matches = cmd.get_matches();
        match matches.subcommand() {
            Some(("ls", _)) => {
                let mut scenarios = self.get_scenario_names();
                scenarios.sort();
                for scenario in scenarios {
                    print!("{}", scenario)
                }
            }
            _ => {}
        }
    }
}

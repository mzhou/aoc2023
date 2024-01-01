use std::collections::{HashMap, VecDeque};
use std::io::stdin;
use std::iter::empty;

use anyhow::{Context, Error};

struct Pulse {
    dst: String,
    pulse_type: PulseType,
    src: String,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum PulseType {
    Low,
    High,
}

impl Default for PulseType {
    fn default() -> Self {
        Self::Low
    }
}

impl PulseType {
    fn flip(&self) -> Self {
        match self {
            PulseType::Low => PulseType::High,
            PulseType::High => PulseType::Low,
        }
    }
}

fn main() -> Result<(), Error> {
    let mut broadcaster_outputs = Vec::<String>::new();
    let mut conjunction_outputs = HashMap::<String, Vec<String>>::new();
    let mut conjunction_inputs = HashMap::<String, HashMap<String, PulseType>>::new(); // last input
    let mut flipflop_outputs = HashMap::<String, Vec<String>>::new();
    let mut flipflop_states = HashMap::<String, PulseType>::new(); // next output

    for line in stdin().lines() {
        let line = line?;
        let Some((module_name, module_output)) = line.split_once(" -> ") else {
            return Err(Error::msg("missing separator"));
        };
        if module_name == "broadcaster" {
            broadcaster_outputs = module_output.split(", ").map(Into::into).collect();
            for output in broadcaster_outputs.iter() {
                conjunction_inputs
                    .entry(output.clone())
                    .or_default()
                    .insert("broadcaster".to_owned(), PulseType::Low);
            }
        } else if module_name.starts_with('%') {
            let name = module_name[1..].to_owned();
            let outputs = module_output
                .split(", ")
                .map(Into::into)
                .collect::<Vec<String>>();
            for output in outputs.iter() {
                conjunction_inputs
                    .entry(output.to_owned())
                    .or_default()
                    .insert(name.to_owned(), PulseType::Low);
            }
            flipflop_outputs.insert(name.to_owned(), outputs);
            flipflop_states.insert(name.to_owned(), PulseType::High);
        } else if module_name.starts_with('&') {
            let name = module_name[1..].to_owned();
            let outputs = module_output
                .split(", ")
                .map(Into::into)
                .collect::<Vec<String>>();
            for output in outputs.iter() {
                conjunction_inputs
                    .entry(output.to_owned())
                    .or_default()
                    .insert(name.to_owned(), PulseType::Low);
            }
            conjunction_outputs.insert(name.to_owned(), outputs);
        }
    }

    let mut total_low = 0;
    let mut total_high = 0;

    for _ in 0..1000 {
        let (low, high) = push_button(
            &broadcaster_outputs,
            &conjunction_outputs,
            &flipflop_outputs,
            &mut conjunction_inputs,
            &mut flipflop_states,
        );
        total_low += low;
        total_high += high;
    }

    println!("{}", total_low * total_high);

    Ok(())
}

fn push_button(
    broadcaster_outputs: &Vec<String>,
    conjunction_outputs: &HashMap<String, Vec<String>>,
    flipflop_outputs: &HashMap<String, Vec<String>>,
    conjunction_inputs: &mut HashMap<String, HashMap<String, PulseType>>,
    flipflop_states: &mut HashMap<String, PulseType>,
) -> (u64, u64) {
    let mut low = 0;
    let mut high = 0;
    let mut pulses = VecDeque::<Pulse>::new();

    pulses.push_back(Pulse {
        dst: "broadcaster".to_owned(),
        pulse_type: PulseType::Low,
        src: "button".to_owned(),
    });

    while let Some(pulse) = pulses.pop_front() {
        match pulse.pulse_type {
            PulseType::Low => low += 1,
            PulseType::High => high += 1,
        }

        if pulse.dst == "broadcaster" {
            for dst in broadcaster_outputs.iter() {
                pulses.push_back(Pulse {
                    dst: dst.to_owned(),
                    pulse_type: pulse.pulse_type,
                    src: "broadcaster".to_owned(),
                });
            }
        } else {
            if let Some(state) = flipflop_states.get_mut(&pulse.dst) {
                if pulse.pulse_type == PulseType::Low {
                    for dst in flipflop_outputs.get(&pulse.dst).unwrap().iter() {
                        pulses.push_back(Pulse {
                            dst: dst.to_owned(),
                            pulse_type: *state,
                            src: pulse.dst.clone(),
                        });
                    }
                    *state = state.flip();
                }
            } else if let Some(state) = conjunction_inputs.get_mut(&pulse.dst) {
                state.insert(pulse.src, pulse.pulse_type);
                let output_type = if state.values().all(|v| *v == PulseType::High) {
                    PulseType::Low
                } else {
                    PulseType::High
                };
                if let Some(dsts) = conjunction_outputs.get(&pulse.dst) {
                    for dst in dsts {
                        pulses.push_back(Pulse {
                            dst: dst.to_owned(),
                            pulse_type: output_type,
                            src: pulse.dst.clone(),
                        });
                    }
                }
            }
        }
    }

    (low, high)
}

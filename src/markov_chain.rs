use std::collections::{HashMap, VecDeque, HashSet};

use nalgebra::DMatrix;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct MarkovChain {
    transition_matrix: DMatrix<f32>,
    index_map: HashMap<String, usize>,
    state_map: HashMap<usize, String>,
}

impl MarkovChain {
    pub fn new(transition_matrix: DMatrix<f32>, states: Vec<String>) -> Self {
        let index_map: HashMap<String, usize> = states
            .iter()
            .cloned()
            .enumerate()
            .map(|v| (v.1, v.0))
            .collect();

        let state_map: HashMap<usize, String> = states
            .iter()
            .cloned()
            .enumerate()
            .map(|v| (v.0, v.1))
            .collect();

        MarkovChain {
            transition_matrix,
            index_map,
            state_map,
        }
    }

    /// Returns the state of random variable at the next time instance.
    pub fn next_state(&self, current_state: String) -> String {
        let mut rng = thread_rng();
        let mut next_state = "".to_string();

        if let Some(i) = self.index_map.get(&current_state) {
            let state_prob = &self.transition_matrix.column(*i);
            let choices: Vec<(String, f32)> = state_prob
                .iter()
                .cloned()
                .enumerate()
                .map(|v| (self.state_map.get(&v.0).unwrap().to_string(), v.1))
                .collect();

            next_state = choices
                .choose_weighted(&mut rng, |item| item.1)
                .unwrap()
                .0
                .clone();
        }

        next_state
    }

    /// Generates the next states of the system.
    pub fn generate_states(&self, current_state: String, no: u32) -> Vec<String> {
        let mut states = vec![];
        let mut current = current_state;

        for _i in 0..no {
            let next = self.next_state(current);
            states.push(next.clone());
            current = next;
        }

        states
    }

    /// Check if state f_state is accessible from i_state
    pub fn is_accessible(&self, i_state: String, f_state: String) -> bool {
        let mut frontier: VecDeque<String> = VecDeque::new();
        let mut explored: HashSet<String> = HashSet::new();

        if let Some(i) = self.index_map.get(&i_state) {
            for k in self.get_states(*i) {
                frontier.push_back(k.to_string());
            }
        }

        while let Some(state) = frontier.pop_front() {
            explored.insert(state.clone());
            if let Some(i) = self.index_map.get(&state) {
                let states: Vec<String> = self.get_states(*i);
                for k in states.iter().clone() {
                    if k == &f_state {
                        return true;
                    } else if frontier.contains(&k.to_string()) == false && explored.contains(&k.to_string()) == false {
                        frontier.push_back(k.to_string());
                    }
                }
            }
        }
        false
    }

    /// Check if the Markov Chain is irreducible
    pub fn is_reducible(&self) -> bool {
        let is_reducible = false;
        let states: Vec<String> = self.index_map.clone().into_iter().map(|(k, v)| k).collect();
        for i in &states {
            for j in &states {
                if !self.is_accessible(i.clone(), j.clone()) {
                    return false;
                }
            }
        }
        true
    }

    fn get_states(&self, i: usize) -> Vec<String> {
        let state_prob = &self.transition_matrix.column(i);
        state_prob
            .iter()
            .cloned()
            .enumerate()
            .filter(|(_, v)| v > &0.0)
            .map(|(k, _)| (self.state_map.get(&k).unwrap().to_string()))
            .collect()
    }
}

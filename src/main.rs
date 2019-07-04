pub mod markov_chain;

use nalgebra::DMatrix;

use crate::markov_chain::MarkovChain;

fn main() {
    let transition_matrix = DMatrix::from_vec(4, 4, vec![
        0.5, 0.5, 0.0, 0.0,
        0.25, 0.0, 0.5, 0.25,
        0.25, 0.5, 0.0, 0.25,
        0.0, 0.0, 0.5, 0.5
    ]);

    /*let transition_matrix = DMatrix::from_vec(4, 4, vec![
        1.0, 1.0, 0.0, 0.0,
        1.0, 1.0, 0.0, 0.0,
        1.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 1.0
    ]);*/

    let states = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
    ];

    let markov_chain = MarkovChain::new(transition_matrix, states);
    let irreducible = markov_chain.is_reducible();

    println!("{:?}", irreducible);
}

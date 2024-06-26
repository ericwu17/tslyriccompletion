//! This code was copied from [https://github.com/justinbarclay/diff](https://github.com/justinbarclay/diff).
//! Thanks to Justin for implementing the Myers Diff Algorithm.
//!
//! An implementation of the greedy version of the Myers Diff Algorithm for Rust.
//! The Algorithm can be described as such:
//! Constant MAX ∈ [0,M+N]
//! Var V: Array [− MAX .. MAX] of Integer
//! 1. V\[1\] ← 0
//! 2. For D ← 0 to MAX Do
//! 3. For k ← −D to D in steps of 2 Do
//! 4. If k = −D or k ≠ D and V[k − 1] < V[k + 1] Then
//! 5. x ← V[k + 1]
//! 6. Else
//! 7. x ← V[k − 1]+1
//! 8. y ← x − k
//! 9. While x < N and y < M and a x + 1 = by + 1 Do (x,y) ← (x+1,y+1)
//! 10. V\[k\] ← x
//! 11. If x ≥ N and y ≥ M Then
//! 12. Length of an SES is D
//! 13. Stop
//! 14. Length of an SES is greater than MAX
//! The full explanation of the algorithm can be found here:time
//! <http://www.xmailserver.org/diff2.pdf>
mod negative_array;

use negative_array::NegativeArray;

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    Insert,
    Delete,
    Null,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A Structure for describing change
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Edit {
    pub edit: Operation,
    pub at: usize,
    pub to: usize,
}

/// A helper function for splitting a string into a vector of characters
fn split_string(string: &str) -> Vec<char> {
    let col: Vec<_> = string.chars().collect();
    col.to_vec()
}

/// The greedy algorithm for diffing strings. Returns a three-tuple of:
/// 1. The number of differences (either insersts of deletes) that occur from String 1 to String 2
/// 2. The last diagonal (K in Meyer's algorithm) that the diffing algorithm ended on.
/// 3. A copy of the history farthest each diagonal reaches in the algorithm given a difference limit.
fn shortest_edit_sequence(
    first: &str,
    second: &str,
) -> Result<(isize, isize, Vec<NegativeArray>), String> {
    let second_chars = split_string(second);
    let first_chars = split_string(first);

    let first_length = first_chars.len() as isize;
    let second_length = second_chars.len() as isize;

    let max = first_length + second_length;

    let mut traversal_history = NegativeArray::new(max);
    traversal_history[1] = 0;

    // history needs to be able to describe differences from [0..Max + 1] so the history needs to hold 2 more elements above max.
    let mut history: Vec<NegativeArray> = vec![NegativeArray::new(0); (max + 2) as usize];

    for d in 0..=max {
        let mut diagonal = -d;
        while diagonal <= d {
            let mut x: isize;
            let mut y: isize;

            let down = diagonal == -d
                || (diagonal != d
                    && traversal_history[diagonal - 1] < traversal_history[diagonal + 1]);
            x = if down {
                traversal_history[diagonal + 1]
            } else {
                traversal_history[diagonal - 1] + 1
            };
            y = x - diagonal;

            while (0 <= x && x < first_length)
                && (0 <= y && y < second_length)
                && (first_chars[x as usize] == second_chars[y as usize])
            {
                x += 1;
                y += 1;
            }

            traversal_history[diagonal] = x;

            if x >= first_length && y >= second_length {
                // Normally we need to push onto history after we've processed everything, but because we've found our match
                // We need to exit
                let final_d = if d % 2 == 0 { d + 1 } else { d };
                history[final_d as usize] = traversal_history.clone();
                return Ok((d, diagonal, history));
            }

            diagonal += 2;
        }

        // We can ignore pushing even slices into the history
        // becuase only an even K will overwrite an even K
        if d % 2 == 1 {
            history[d as usize] = traversal_history.clone();
        }
    }
    Err("Failed To Find Shortest Edit Sequence".to_string())
}

// A helper function for generating an edit graph based on the history of edits returned from
// `shortest_edit_sequence`. This edit graph represent a discrete set of operations that is
// needed to transform the string `first` to the string `second`.
fn generate_edit_graph_loop(
    first: &str,
    second: &str,
    diff: isize,
    original_diagonal: isize,
    history: Vec<NegativeArray>,
) -> Result<Vec<Edit>, String> {
    // set constants to match algo
    if diff == -1 {
        return Ok(vec![]);
    }
    let first_length = first.chars().count() as isize;
    let second_length = second.chars().count() as isize;

    let second_chars = split_string(second);
    let first_chars = split_string(first);

    // Things we will need access to later
    let mut difference = diff;
    let mut edit_graph = Vec::with_capacity(difference as usize);
    let mut op: Edit;
    let mut new_diagonal: isize;
    let mut diagonal = original_diagonal;

    // Controlling borrowing scope by creating a closure
    while difference > -1 {
        let furthest_path_at_d = if difference % 2 == 0 {
            &history[(difference + 1) as usize]
        } else {
            &history[difference as usize]
        };
        // Let's set some state we need access outside of loop
        let mut best_diagonal = None;
        let mut best_y = -1;
        let mut best_x = -1;
        // Because we're traversing our history. We know as we step back in the history
        // that to get to our current K it must be through either an insert or a delete.
        // So it must be the K directly above us or below us.

        // Let's find which operation, inserting or deleting gets us farther
        for position in [diagonal - 1, diagonal + 1].iter() {
            let mut x = furthest_path_at_d[*position];
            let mut y = x - position;

            while (0 <= x && x < first_length)
                && (0 <= y && y < second_length)
                && first_chars[x as usize] == second_chars[y as usize]
            {
                x += 1;
                y += 1;
            }
            if x > best_x {
                best_diagonal = Some(*position);
                best_x = furthest_path_at_d[*position];
                best_y = best_x - position;
            }
        }

        new_diagonal = match best_diagonal {
            // This is ugly we should extract this to a function to hide it
            Some(diagonal) => diagonal,
            None => {
                return Err("Failed to Generate Edit Graph".to_string());
            }
        };
        op = if new_diagonal == diagonal + 1 {
            Edit {
                edit: Operation::Insert,
                at: best_y as usize,
                to: best_y as usize,
            }
        } else {
            Edit {
                edit: Operation::Delete,
                at: best_x as usize,
                to: best_x as usize,
            }
        };
        edit_graph.push(op);

        difference -= 1;
        diagonal = new_diagonal;
    }

    edit_graph.reverse();
    Ok(edit_graph)
}

/// Simplifies an edit graph into a set of contiguous operations that describes a series of deletions
/// or insertion.
fn simplify_edit_graph(edit_graph: Vec<Edit>) -> HashMap<String, Vec<Edit>> {
    let mut map: HashMap<String, Vec<Edit>> = HashMap::new();
    map.insert(String::from("insert"), Vec::new());
    map.insert(String::from("delete"), Vec::new());

    let mut previous_edit = Edit {
        edit: Operation::Null,
        at: 0,
        to: 0,
    };

    for edit in edit_graph {
        let operation_string = match edit.edit {
            Operation::Insert => String::from("insert"),
            Operation::Delete => String::from("delete"),
            Operation::Null => String::from("null"),
        };
        // If previous edit matches the same type of our current edit node and the current edit
        // nodes start position is one more than the previous edit nodes then we can add onto
        // our last edit
        if previous_edit.edit == edit.edit && edit.at > 0 && previous_edit.at == edit.at - 1 {
            let mut edit_range = map.get_mut(&operation_string).unwrap().pop().unwrap();

            edit_range.to = edit.at;
            map.get_mut(&operation_string).unwrap().push(edit_range);
        } else {
            map.get_mut(&operation_string).unwrap().push(edit);
        }
        previous_edit = edit;
    }

    map
}

// A function to highlight the differences in a string. Deletions will be hunted in red
// and insertions highlighted in green
#[allow(dead_code)]
pub fn decorate_differences(string: &str, edit_type: &str, edits: &[Edit]) -> String {
    let red = "\x1b[31m";
    let end_colour = "\x1b[0m";
    let green = "\x1b[32m";

    let colour = if edit_type == "insert" { green } else { red };
    let starting_symbol = if edit_type == "insert" { "+ " } else { "- " };
    let mut response = String::new();

    response.push_str(colour);
    response.push_str(starting_symbol);
    response.push_str(end_colour);

    if edits.is_empty() {
        response.push_str(string);
    } else {
        let mut edits_1 = edits.to_vec();
        edits_1.reverse();
        let mut maybe_edit = edits_1.pop();

        for (index, character) in string.chars().enumerate() {
            match maybe_edit {
                Some(edit) => {
                    if index == edit.at {
                        response.push_str(colour);
                    }
                    response.push(character);
                    if index == edit.to {
                        response.push_str(end_colour);
                        maybe_edit = edits_1.pop();
                    }
                }
                None => response.push(character),
            }
        }
    }

    response
}

/// The Meyer's greedy string diffing alorithms. Returns a HahsMap of Edit
/// describing: what positions in the first string that need to be deleted to match String 2,
/// under the `delete` key, and what positions in the second string need to be inserted into the first string
/// to trnasform String 1 into String 2, under the `insert` key
pub fn diff_greedy(first: &str, second: &str) -> Result<(i32, HashMap<String, Vec<Edit>>), String> {
    // Let's save ourselves some time calls if possible
    if first.is_empty() && second.is_empty() {
        // If both strings are empty return a hashmap with empty vectors
        let mut map: HashMap<String, Vec<Edit>> = HashMap::new();
        map.insert(String::from("insert"), Vec::new());
        map.insert(String::from("delete"), Vec::new());
        Ok((0, map))
    } else if first.is_empty() && !second.is_empty() {
        // If first is empty and second isn't, all characters in the second are inserts
        let mut map: HashMap<String, Vec<Edit>> = HashMap::new();
        map.insert(
            String::from("insert"),
            vec![
                Edit {
                    edit: Operation::Insert,
                    at: 0,
                    to: second.len()
                };
                1
            ],
        );
        map.insert(String::from("delete"), Vec::new());
        Ok((second.len() as i32, map))
    } else if !first.is_empty() && second.is_empty() {
        // Like wise if second is empty and first isn't, all characters in the first are deletes
        let mut map: HashMap<String, Vec<Edit>> = HashMap::new();
        map.insert(
            String::from("delete"),
            vec![
                Edit {
                    edit: Operation::Delete,
                    at: 0,
                    to: first.len()
                };
                1
            ],
        );
        map.insert(String::from("insert"), Vec::new());
        Ok((first.len() as i32, map))
    } else {
        // Sadly we have to do some work now
        let (difference, diagonal, history) = shortest_edit_sequence(first, second)?;

        let edit_graph =
            generate_edit_graph_loop(first, second, difference - 1, diagonal, history)?;

        let simple_edit_graph = simplify_edit_graph(edit_graph);

        Ok((difference as i32, simple_edit_graph))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn split_string_hello() {
        let split = split_string("Hello");
        let pre_split = vec!['H', 'e', 'l', 'l', 'o'];
        assert!(pre_split.len() == split.len());
        assert_eq!(split, pre_split);
    }

    #[test]
    fn short_edit_sequence() {
        let history = vec![
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 5,
                arr: [-1, -1, -1, -1, 2, 1, 0, -1, -1, -1, -1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
        ];
        let result = shortest_edit_sequence("H\n", "Hi\n").unwrap();
        assert_eq!(result.0, 1);
        assert_eq!(result.1, -1);
        assert_eq!(result.2, history);
    }

    #[test]
    fn short_edit_sequence_where_they_are_the_same() {
        let history = vec![
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 10,
                arr: [
                    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 5, 0, -1, -1, -1, -1, -1, -1, -1, -1,
                    -1,
                ]
                .to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
        ];
        let result = shortest_edit_sequence("Hello", "Hello").unwrap();
        assert_eq!(result.0, 0);
        assert_eq!(result.1, 0);
        assert_eq!(result.2, history);
    }

    #[test]
    fn a_short_squence_where_they_are_the_same() {
        let history = vec![
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 2,
                arr: [-1, -1, 1, 0, -1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
        ];
        let result = shortest_edit_sequence("H", "H").unwrap();
        assert_eq!(result.0, 0);
        assert_eq!(result.1, 0);
        assert_eq!(result.2, history);
    }

    #[test]
    fn gen_an_edit_sequence_for_a_diff_of_zero() {
        let history = vec![
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 2,
                arr: [-1, -1, 1, 0, -1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
        ];
        let edit_graph = vec![];
        let result = generate_edit_graph_loop("H", "H", -1, 0, history).unwrap();
        assert_eq!(result, edit_graph);
    }

    #[test]
    fn short_edit_sequence_where_nothing_matches() {
        let result = shortest_edit_sequence("Hze", "Nod").unwrap();
        assert_eq!(result.0, 6);
        assert_eq!(result.1, 0);
    }

    #[test]
    fn short_edit_sequence_for_empty_string() {
        let result = shortest_edit_sequence("", "1").unwrap();
        assert_eq!(result.0, 1);
        assert_eq!(result.1, -1);
    }

    #[test]
    fn gen_edit_graph() {
        let history = vec![
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 5,
                arr: [-1, -1, -1, -1, 2, 1, 0, -1, -1, -1, -1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
        ];
        let edit_graph = vec![Edit {
            edit: Operation::Insert,
            at: 1,
            to: 1,
        }];
        let result = generate_edit_graph_loop("H\n", "Hi\n", 0, -1, history).unwrap();
        assert_eq!(edit_graph, result);
    }

    #[test]
    fn short_edit_sequence_without_newlines() {
        let history = vec![
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 3,
                arr: [-1, -1, 1, 1, 0, -1, -1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
        ];
        let result = shortest_edit_sequence("H", "Hi").unwrap();
        assert_eq!(result.0, 1);
        assert_eq!(result.1, -1);
        assert_eq!(result.2, history);
    }

    #[test]
    fn gen_edit_graph_without_newlines() {
        let history = vec![
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 3,
                arr: [-1, -1, 1, 1, 0, -1, -1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
            NegativeArray {
                max: 0,
                arr: [-1].to_vec(),
            },
        ];
        let edit_graph = vec![Edit {
            edit: Operation::Insert,
            at: 1,
            to: 1,
        }];
        let result = generate_edit_graph_loop("H\n", "Hi\n", 0, -1, history).unwrap();
        assert_eq!(edit_graph, result);
    }

    #[test]
    fn greedy_diff() {
        let mut expected_differences: HashMap<String, Vec<Edit>> = HashMap::new();
        expected_differences.insert(String::from("insert"), Vec::new());
        expected_differences.insert(String::from("delete"), Vec::new());
        expected_differences.get_mut("insert").unwrap().push(Edit {
            edit: Operation::Insert,
            at: 1,
            to: 1,
        });
        let (number_of_differences, differences) = diff_greedy("H", "Hi").unwrap();
        assert_eq!(number_of_differences, 1);
        assert_eq!(differences, expected_differences);
    }
}

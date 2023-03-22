//------------------------------------------------------------------------------------------
//  Copyright (c) 2023 Finnian Brown. All rights reserved.
//  Licensed under the GNU General Public License v3.0. See LICENSE in the project root.
//------------------------------------------------------------------------------------------

use rand::seq::SliceRandom;
use std::borrow::Cow;
use std::collections::VecDeque;
use std::time::Instant;

const MAZE_WIDTH: usize = 25;
const MAZE_HEIGHT: usize = 25;

/// Represents a cell in the maze.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Path,
    Visited,
    Solution,
}

fn main() {
    let width = MAZE_WIDTH;
    let height = MAZE_HEIGHT;
    let (mut maze, end) = generate_maze(width, height);
    display_maze(&maze);

    println!("\nSolving maze...\n");

    let start = (1, 1);
    let start_time = Instant::now();

    let solution = solve_maze(&mut maze, start, end);
    let duration = start_time.elapsed();

    if let Some(path) = solution {
        println!("Solution Found in: {:?}", duration);
        for (x, y) in path {
            maze[y][x] = Cell::Solution;
        }
        display_maze(&maze);
    } else {
        println!("No solution found.");
    }
}

/// Generates a random maze of the specified width and height.
///
/// # Arguments
///
/// * `width` - The width of the maze.
/// * `height` - The height of the maze.
///
/// # Returns
///
/// A tuple containing a 2D vector of `Cell` representing the generated maze
/// and a tuple (x, y) representing the ending coordinates.
fn generate_maze(width: usize, height: usize) -> (Vec<Vec<Cell>>, (usize, usize)) {
    let mut maze: Vec<Vec<Cell>> = vec![vec![Cell::Wall; width]; height];

    maze[1][1] = Cell::Path;
    carve_passage(1, 1, &mut maze);

    // Find a suitable end position
    let end = (height - 2, width - 2);
    if maze[end.1][end.0] == Cell::Wall {
        for y in (0..height).rev() {
            for x in (0..width).rev() {
                if maze[y][x] == Cell::Path {
                    return (maze, (x, y));
                }
            }
        }
    }

    (maze, end)
}

/// Carves passages in the maze using a depth-first algorithm.
///
/// This function takes the current position (`x`, `y`) and a mutable reference to the maze.
/// It randomizes the order of possible directions and then checks if the new position is valid
/// and is a `Wall`. If so, it creates a `Path` in the maze and calls itself recursively with the
/// new position.
///
/// # Arguments
///
/// * `x` - The x-coordinate of the current position in the maze.
/// * `y` - The y-coordinate of the current position in the maze.
/// * `maze` - A mutable reference to a 2D vector of `Cell` representing the maze.
///
/// # Side Effects
///
/// Modifies the `maze` to create passages.
fn carve_passage(x: usize, y: usize, maze: &mut Vec<Vec<Cell>>) {
    let mut rng = rand::thread_rng();
    let directions: [(isize, isize); 4] = [(0, -2), (0, 2), (-2, 0), (2, 0)];
    let mut possible_directions = directions.to_vec();
    possible_directions.shuffle(&mut rng);

    for (dx, dy) in &possible_directions {
        let new_x = (x as isize + *dx) as usize;
        let new_y = (y as isize + *dy) as usize;

        if new_x > 0 && new_x < maze[0].len() - 1 && new_y > 0 && new_y < maze.len() - 1 {
            if maze[new_y][new_x] == Cell::Wall {
                maze[(y as isize + *dy / 2) as usize][(x as isize + *dx / 2) as usize] = Cell::Path;
                maze[new_y][new_x] = Cell::Path;
                carve_passage(new_x, new_y, maze);
            }
        }
    }
}

/// Displays the maze on the console.
///
/// # Arguments
///
/// * `maze` - A reference to a 2D vector of `Cell` representing the maze.
fn display_maze(maze: &Vec<Vec<Cell>>) {
    for row in maze {
        let line: String = row
            .iter()
            .map(|cell| -> Cow<str> {
                match cell {
                    Cell::Wall => Cow::Borrowed("█"),
                    Cell::Path => Cow::Borrowed(" "),
                    Cell::Visited => Cow::Borrowed("."),
                    Cell::Solution => Cow::Owned(String::from("\u{001b}[32m*\u{001b}[0m")), // Green solution character
                }
            })
            .collect();
        println!("{}", line);
    }
}

/// Solves the maze using depth-first search.
///
/// # Arguments
///
/// * `maze` - A mutable reference to a 2D vector of `Cell` representing the maze.
/// * `start` - A tuple (x, y) representing the starting coordinates.
/// * `end` - A tuple (x, y) representing the ending coordinates.
///
/// # Returns
///
/// An `Option` containing a `VecDeque` of (x, y) tuples representing the solution path,
/// or `None` if no solution is found.
fn solve_maze(
    maze: &mut Vec<Vec<Cell>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<VecDeque<(usize, usize)>> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut stack = VecDeque::new();
    stack.push_front((start, VecDeque::new()));

    while let Some(((x, y), mut path)) = stack.pop_front() {
        if maze[y][x] == Cell::Path {
            maze[y][x] = Cell::Visited;
            path.push_front((x, y));

            if (x, y) == end {
                path.pop_front(); // Remove the last cell from the path, as it's the end position.
                return Some(path);
            }

            for (dx, dy) in &directions {
                let new_x = (x as isize + *dx) as usize;
                let new_y = (y as isize + *dy) as usize;
                if new_x < maze[0].len() && new_y < maze.len() {
                    stack.push_front(((new_x, new_y), path.clone()));
                }
            }
        }
    }

    None
}

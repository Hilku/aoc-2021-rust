use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("day15input.txt");
    let mut parts = input.lines();

    let mut risk_levels: Vec<Vec<usize>> = Vec::new();
    let mut index = 0;
    while let Some(line) = parts.next() {
        risk_levels.push(Vec::new());
        for c in line.chars() {
            risk_levels[index].push(c.to_digit(10).unwrap() as usize);
        }
        index += 1;
    }

    println!("Part 1 Dijkstra: {:?}", dijkstra(&risk_levels));
    let tile_size = (risk_levels.len(), risk_levels[0].len());

    let mut inflated_risk_levels = risk_levels.clone();

    //INFLATE
    for risk_level_col in inflated_risk_levels.iter_mut() {
        for i in 0..4 {
            for col in 0..tile_size.0 {
                let mut new_risk = risk_level_col[col + i * tile_size.0] + 1;
                if new_risk > 9 {
                    new_risk = 1;
                }
                risk_level_col.push(new_risk);
            }
        }
    }
    for i in 0..4 {
        for j in 0..tile_size.0 {
            inflated_risk_levels.push(Vec::new());
            let risk_levels_len = inflated_risk_levels.len() - 1;
            let row_len = inflated_risk_levels[j + i * tile_size.0].len();
            for col in 0..row_len {
                let mut new_risk = inflated_risk_levels[j + i * tile_size.0][col] + 1;
                if new_risk > 9 {
                    new_risk = 1;
                }
                inflated_risk_levels[risk_levels_len].push(new_risk);
            }
        }
    }

    println!("Dijkstra: {:?}", dijkstra(&inflated_risk_levels));
}

fn dijkstra(risk_levels: &Vec<Vec<usize>>) -> Option<usize> {
    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; risk_levels[0].len()]; risk_levels.len()];
    let mut heap = BinaryHeap::new();
    let goal = (risk_levels.len() - 1, risk_levels[0].len() - 1);
    dist[0][0] = 0;
    heap.push(State {
        cost: 0,
        position: (0, 0),
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position.0][position.1] {
            continue;
        }

        for neighbour in get_neighbours(&risk_levels, position) {
            let next = State {
                cost: cost + risk_levels[neighbour.0][neighbour.1],
                position: neighbour,
            };

            if next.cost < dist[next.position.0][next.position.1] {
                heap.push(next);
                dist[next.position.0][next.position.1] = next.cost;
            }
        }
    }

    None
}

fn get_neighbours(risk_levels: &Vec<Vec<usize>>, position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    if position.0 > 0 {
        neighbours.push((position.0 - 1, position.1));
    }
    if position.0 < risk_levels.len() - 1 {
        neighbours.push((position.0 + 1, position.1));
    }
    if position.1 > 0 {
        neighbours.push((position.0, position.1 - 1));
    }
    if position.1 < risk_levels[position.0].len() - 1 {
        neighbours.push((position.0, position.1 + 1));
    }

    return neighbours;
}

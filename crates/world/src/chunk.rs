use std::collections::{HashSet, VecDeque};

use cellophanemc_core::chunk_pos::ChunkPos;

pub fn generate_bfs_order(radius: usize) -> Vec<ChunkPos> {
    let mut chunks = Vec::new();
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    seen.insert(ChunkPos::new(0, 0));
    queue.push_back(ChunkPos::new(0, 0));

    while !queue.is_empty() {
        if let Some(chunk) = queue.pop_front() {
            // Important that the addition to the list is here, rather than when enqueuing neighbors.
            // Ensures the order is actually kept.
            chunks.push(chunk);

            for neighbor in vec![
                ChunkPos::new(chunk.x - 1, chunk.z),
                ChunkPos::new(chunk.x, chunk.z - 1),
                ChunkPos::new(chunk.x + 1, chunk.z),
                ChunkPos::new(chunk.x, chunk.z + 1),
            ] {
                if neighbor.x.abs().max(neighbor.z.abs()) > (radius as i32) {
                    // Don't enqueue out of range.
                    continue;
                }
                if !seen.insert(neighbor) {
                    continue;
                }
                queue.push_back(neighbor);
            }
        }
    }

    // first, build a map of manhatten distance -> chunks
    let mut by_distance = vec![];
    let chunk_len = chunks.len();
    for chunk in chunks {
        let dist = (chunk.x.abs() as usize) + (chunk.z.abs() as usize);
        if dist == by_distance.len() {
            by_distance.push(vec![chunk; 1]);
            continue;
        }

        by_distance[dist].push(chunk);
    }

    // per distance we transform the chunk list so that each element is maximally spaced out from each other
    for i in 0..by_distance.len() {
        let mut not_added = by_distance[i].clone();
        let mut added = vec![];

        while !not_added.is_empty() {
            if added.is_empty() {
                &added.push(not_added.pop().unwrap());
                continue;
            }

            let mut max_chunk = None;
            let mut max_dist = 0;

            for chunk in not_added.iter() {
                let mut min_dist = i32::MAX;

                for added_chunk in added.iter() {
                    let dist = (added_chunk.x - chunk.x).abs().max((added_chunk.z - chunk.z).abs());

                    if dist < min_dist {
                        min_dist = dist;
                    }
                }

                if min_dist > max_dist {
                    max_dist = min_dist;
                    max_chunk = Some(chunk);
                }
            }
            let max_chunk = max_chunk.unwrap().clone();

            not_added.retain(|x| !(x.x == max_chunk.x && x.z == max_chunk.z));

            &added.push(max_chunk);
        }

        by_distance[i] = added;
    }

    let mut result = Vec::with_capacity(chunk_len);
    for x in by_distance.iter() {
        result.extend_from_slice(x);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::chunk::generate_bfs_order;

    #[test]
    fn foo() {
        env_logger::init();
        let mut search_radius_iteration_list = vec![];
        for i in 0..64 {
            search_radius_iteration_list.push(generate_bfs_order(i))
        }
        for x in search_radius_iteration_list.iter().enumerate() {
            println!("{} {:?}", x.0, x.1);
        }
    }
}

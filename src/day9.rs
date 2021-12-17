pub type HeightMap = Vec<Vec<i32>>;

/// Surround the heightmap with rows and columns of 9 to make the processing stage easier. This way
/// just need to check the "inner" real map
#[aoc_generator(day9)]
fn heightmap(input: &str) -> HeightMap {
    const PAD: i32 = 9;
    let mut real_rows = input
        .lines()
        .map(|line| {
            let mut heights = line
                .trim()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as i32)
                .collect::<Vec<_>>();
            let mut row = vec![PAD];
            row.append(&mut heights);
            row.push(PAD);
            row
        })
        .collect::<Vec<_>>();

    let cols = real_rows.first().unwrap().len();
    let mut height_map = Vec::with_capacity(cols);
    height_map.push(vec![PAD; cols]);
    height_map.append(&mut real_rows);
    height_map.push(vec![PAD; cols]);
    height_map
}

pub fn is_low_point(heightmap: &HeightMap, row: usize, col: usize) -> bool {
    let center = heightmap[row][col];
    let left = heightmap[row][col - 1];
    let right = heightmap[row][col + 1];
    let above = heightmap[row - 1][col];
    let below = heightmap[row + 1][col];
    center < left && center < right && center < above && center < below
}

pub fn find_lowpoints(heightmap: &HeightMap) -> Vec<(usize, usize)> {
    let rows = heightmap.len() - 1;
    let cols = heightmap.first().unwrap().len() - 1;

    (1..rows)
        .flat_map(|row| {
            (1..cols).filter_map(move |col| {
                if is_low_point(heightmap, row, col) {
                    Some((row, col))
                } else {
                    None
                }
            })
        })
        .collect()
}

pub fn basin_size(heightmap: &HeightMap, row: usize, col: usize) -> i32 {
    let rows = heightmap.len();
    let cols = heightmap.first().unwrap().len();
    let mut visited = vec![vec![false; cols]; rows];

    floodfill(heightmap, row, col, &mut visited)
}

pub fn floodfill(
    heightmap: &HeightMap,
    row: usize,
    col: usize,
    visited: &mut Vec<Vec<bool>>,
) -> i32 {
    let center = heightmap[row][col];
    if center >= 9 || visited[row][col] {
        return 0;
    }
    visited[row][col] = true;

    1 + floodfill(heightmap, row, col - 1, visited)   // left
        + floodfill(heightmap, row, col + 1, visited) // right
        + floodfill(heightmap, row -1, col, visited)  // top
        + floodfill(heightmap, row + 1, col, visited) // bottom
}

#[aoc(day9, part1)]
fn part1(heights: &HeightMap) -> i32 {
    let low_points = find_lowpoints(heights);
    low_points
        .iter()
        .fold(0, |sum, &(row, col)| 1 + sum + heights[row][col])
}

#[aoc(day9, part2)]
fn part2(heights: &HeightMap) -> i32 {
    let low_points = find_lowpoints(heights);
    let mut basin_sizes = low_points
        .iter()
        .map(|&(row, col)| basin_size(heights, row, col))
        .collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = heightmap(
            r"2199943210
3987894921
9856789892
8767896789
9899965678",
        );
        assert_eq!(part1(&input), 15);
        assert_eq!(part2(&input), 1134);
    }
}

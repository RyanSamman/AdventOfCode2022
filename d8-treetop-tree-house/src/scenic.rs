use std::fmt::Debug;

pub type TreeGrid<T> = Vec<Vec<T>>;

pub fn create_empty_grid<T>(size: usize, default: T) -> TreeGrid<T>
where
    T: Copy,
{
    let mut map = Vec::new();

    for i in 0..size {
        map.push(Vec::new());
        for _ in 0..size {
            map[i].push(default);
        }
    }

    map
}

pub fn print_grid<T>(g: &TreeGrid<T>)
where
    T: Debug,
{
    g.iter().for_each(|x| println!("{x:?}"));
}

pub fn left_map(tree_grid: &TreeGrid<i32>) -> TreeGrid<i32> {
    let size = tree_grid.len();
    let mut map = create_empty_grid(size, 0);

    for i in 0..size {
        for j in (0..size).rev() {
            let mut visibility_count = 0;

            for k in (0..j).rev() {
                visibility_count += 1;
                if tree_grid[i][k] >= tree_grid[i][j] {
                    break;
                }
            }

            map[i][j] = visibility_count;
        }
    }

    map
}

pub fn right_map(tree_grid: &TreeGrid<i32>) -> TreeGrid<i32> {
    let size = tree_grid.len();
    let mut map = create_empty_grid(size, 0);

    for i in 0..size {
        for j in 0..size {
            let mut visibility_count = 0;

            for k in j+1..size {
                visibility_count += 1;
                if tree_grid[i][k] >= tree_grid[i][j] {
                    break;
                }
            }

            map[i][j] = visibility_count;
        }
    }

    map
}

pub fn top_map(tree_grid: &TreeGrid<i32>) -> TreeGrid<i32> {
    let size = tree_grid.len();
    let mut map = create_empty_grid(size, -1);

    for i in 0..size {
        for j in (0..size).rev() {
            let mut visibility_count = 0;

            for k in (0..j).rev() {
                visibility_count += 1;
                if tree_grid[k][i] >= tree_grid[j][i] {
                    break;
                }
            }

            map[j][i] = visibility_count;
        }
    }

    map
}

pub fn bottom_map(tree_grid: &TreeGrid<i32>) -> TreeGrid<i32> {
    let size = tree_grid.len();
    let mut map = create_empty_grid(size, -1);

    for i in 0..size {
        for j in 0..size {
            let mut visibility_count = 0;

            for k in j+1..size {
                visibility_count += 1;
                if tree_grid[k][i] >= tree_grid[j][i] {
                    break;
                }
            }

            map[j][i] = visibility_count;
        }
    }

    map
}

pub fn mult_maps(mps: &[&TreeGrid<i32>]) -> TreeGrid<i32> {
    let size = mps[0].len();
    let mut map = create_empty_grid(size, -1);

    // Iterate through rows
    for i in 0..size {
        // Iterate through Columns
        // From Left to Right
        for j in (0..size).rev() {
            map[i][j] = mps.iter()
                        .map(|x| x[i][j])
                        .fold(1, |acc, c| acc * c);
        }
    }

    map
}

pub fn find_most_scenic_value(scenic_tree_grid: &TreeGrid<i32>) -> i32 {
    *scenic_tree_grid.iter().flatten().max().unwrap_or(&-1)
}

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

    // Iterate through rows
    for i in 0..size {
        let mut prev = -1;

        // Iterate through Columns
        // From Left to Right
        for j in (0..size).rev() {
            let current = tree_grid[i][j];
            map[i][j] = prev;
            prev = current.max(prev);
        }
    }

    map
}

pub fn right_map(tree_grid: &TreeGrid<i32>) -> TreeGrid<i32> {
    let size = tree_grid.len();
    let mut map = create_empty_grid(size, 0);

    // Iterate through rows
    for i in 0..size {
        let mut prev = -1;

        // Iterate through Columns
        // From Left to Right
        for j in 0..size {
            let current = tree_grid[i][j];
            map[i][j] = prev;
            prev = current.max(prev);
        }
    }

    map
}

pub fn top_map(tree_grid: &TreeGrid<i32>) -> TreeGrid<i32> {
    let size = tree_grid.len();
    let mut map = create_empty_grid(size, -1);

    // Iterate through rows
    for i in 0..size {
        let mut prev = -1;

        // Iterate through Columns
        // From Left to Right
        for j in (0..size).rev() {
            let current = tree_grid[j][i];
            map[j][i] = prev;
            prev = current.max(prev);
        }
    }

    map
}

pub fn bottom_map(tree_grid: &TreeGrid<i32>) -> TreeGrid<i32> {
    let size = tree_grid.len();
    let mut map = create_empty_grid(size, -1);

    // Iterate through rows
    for i in 0..size {
        let mut prev = -1;

        // Iterate through Columns
        // From Left to Right
        for j in 0..size {
            let current = tree_grid[j][i];
            map[j][i] = prev;
            prev = current.max(prev);
        }
    }

    map
}

pub fn min_maps(mps: &[&TreeGrid<i32>]) -> TreeGrid<i32> {
    let size = mps[0].len();
    let mut map = create_empty_grid(size, -1);

    // Iterate through rows
    for i in 0..size {
        // Iterate through Columns
        // From Left to Right
        for j in (0..size).rev() {
            map[i][j] = mps.iter()
                            .map(|x| x[i][j])
                            .min().unwrap();
        }
    }

    map
}

pub fn apply_visibility_map(tree_grid: &TreeGrid<i32>, mm: &TreeGrid<i32>) -> TreeGrid<bool> {
    let size = mm[0].len();
    let mut map = create_empty_grid(size, false);

    // Iterate through rows
    for i in 0..size {
        // Iterate through Columns
        // From Left to Right
        for j in (0..size).rev() {
            map[i][j] = tree_grid[i][j] > mm[i][j];
        }
    }

    map
}

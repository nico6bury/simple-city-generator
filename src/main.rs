use grid::{Grid};
use rand::{Rng};

fn main() {
    // create our empty grid
    let mut city_grid: Grid<String> = create_empty_grid(10, 10);
    // show empty grid
    println!("Empty Grid:");
    print_grid(&city_grid);

    // pick the groups we'll use
    let mut groups: Vec<&str> = Vec::new();
    groups.push("slum");
    groups.push("suburb");
    groups.push("adventuring");
    groups.push("financial");
    groups.push("business");

    // add group starts in random spots
    city_grid = prime_grid_with_groups(city_grid, groups);
    // show additions to grid
    println!("Primed Grids:");
    print_grid(&city_grid);
    
}//end main function

/// # prime_grid_with_groups()
/// 
/// Adds single instance of each group in random spots in the grid.
fn prime_grid_with_groups(mut grid:Grid<String>, groups:Vec<&str>) -> Grid<String> {
    // create a random number generator to use
    let mut rng = rand::thread_rng();
    // start looping through groups to actually do stuff
    for group in groups {
        loop {
            // generate random location
            let row = rng.gen_range(0..grid.rows());
            let col = rng.gen_range(0..grid.cols());
            // check that we're not overlapping
            if grid.get(row, col).unwrap().eq("empty") {
                // actually put the group in
                let spot = grid.get_mut(row, col).unwrap();
                *spot = group.to_string();
                break;
            }//end if we can continue
            else {continue;}
        }//end while we need to do check to not overwrite other group
    }//end generating something for each group

    return grid;
}//end prime_grid_with_groups

/// # print_grid()
/// 
/// This function prints the specified grid to the console for debugging purposes.
fn print_grid(grid:&Grid<String>) {
    for row in grid.iter_rows() {
        for item in row {
            let mut shrunk = item.as_str();
            if shrunk.len() > 6 {
                shrunk = &item[0..6];
            }//end if we need to shrink the name
            print!("{}\t", shrunk);
        }//end looping over elements in row
        print!("\n");
    }//end looping over rows
    print!("\n");
}//end printGrid(grid)

/// # create_empty_grid()
/// 
/// This function creates an empty grid of the specified dimensions, filled with the string "empty".
fn create_empty_grid(rows:usize, cols:usize) -> Grid<String> {
    let mut empty = Grid::new(rows, cols);
    empty.fill("empty".to_string());
    return empty;
}//end createEmptyGrid
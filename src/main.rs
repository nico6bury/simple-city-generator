use grid::Grid;
use rand::Rng;
mod grouping;
use grouping::Coord;
use grouping::Grouping;

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
    // create the grouping list
    let mut groupings: Vec<Grouping> = Vec::new();
    for group in groups {
        groupings.push(Grouping::new(group.to_string()));
    }//end adding each group to grouping

    // add group starts in random spots
    prime_grid_with_groups(&mut city_grid, &mut groupings);
    // show additions to grid
    println!("Primed Grids:");
    print_grid(&city_grid);
    // print out the groupings
    println!("Current Groupings");
    for group in &groupings {
        println!("{}", group.to_string());
    }//end printing out each grouping
    
}//end main function

/// # prime_grid_with_groups()
/// 
/// Adds single instance of each group in random spots in the grid.
fn prime_grid_with_groups(grid:&mut Grid<String>, groups:&mut Vec<Grouping>){
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
                *spot = group.name.clone();
                // update the grouping
                group.locations.push(Coord::new(row, col));
                break;
            }//end if we can continue
            else {continue;}
        }//end while we need to do check to not overwrite other group
    }//end generating something for each group
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
use grid::Grid;
use rand::Rng;
mod grouping;
use grouping::Coord;
use grouping::Grouping;
use rand::rngs::ThreadRng;

fn main() {
    // create random number generator for whole program
    let mut rng = rand::thread_rng();
    // create our empty grid
    let mut city_grid: Grid<String> = create_empty_grid(10, 10);
    // // show empty grid
    // print_grid(&city_grid, "Empty Grids");

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
    prime_grid_with_groups(&mut city_grid, &mut groupings, &mut rng);
    // // show additions to grid
    // print_grid(&city_grid, "Primed Grids");
    // // print out the groupings
    // print_groupings(&groupings, "Current Groupings");

    // advance groups until enclosed
    let mut all_enclosed = false;
    while !all_enclosed {
        let num_enclosed = advance_group_expansion(&mut city_grid, &mut groupings, &mut rng);
        all_enclosed = num_enclosed.eq(&groupings.len());
    }//end looping while some groupings are still able to expand

    // show additions to grid
    print_grid(&city_grid, "Advanced Groupings");
    // print out the groupings
    print_groupings(&groupings, "Current Groupings");
    
}//end main function

/// # advance_group_expansion(grid, groups)
/// 
/// For each group, an adjacent, unclaimed tile will be claimed. If no tiles can be claimed, the group will not expand.
/// 
/// ## Return
/// This function returns the number of groups which could not be expanded because they were completely enclosed
fn advance_group_expansion(grid:&mut Grid<String>, groups:&mut Vec<Grouping>, rng:&mut ThreadRng) -> usize {
    // counter to keep track of fully enclosed groups
    let mut num_enclosed: usize = 0;
    // start looping through the groups to advance
    for group in groups {
        // find the coords which are adjacent to group and unclaimed by any other group
        let adjacent_coords = group.get_adjacent_coords(grid.rows() - 1, grid.cols() - 1, false);
        let mut open_coords = Vec::new();
        for coord in adjacent_coords {
            if grid.get(coord.row, coord.col).unwrap().eq("empty") {
                open_coords.push(coord);
            }//end if the coord is still unclaimed
        }//end checking each coord in adjacent_coords to add to open_coords

        // don't try to advance if there's no valid open coords
        if open_coords.len() == 0 { num_enclosed += 1; continue;}
        // pick one of the open_coords
        // TODO: add better spot-picking bias to normalize group shapes
        let coord_to_use = open_coords.get(rng.gen_range(0..open_coords.len())).unwrap();
        // update group name in grid
        let grid_spot = grid.get_mut(coord_to_use.row, coord_to_use.col).unwrap();
        *grid_spot = group.name.clone();
        // update grouping locations
        group.locations.push(coord_to_use.clone());
    }//end looping over each group to advance
    return num_enclosed;
}//end advance_group_expansion(grid, groups)

/// # prime_grid_with_groups()
/// 
/// Adds single instance of each group in random spots in the grid.
fn prime_grid_with_groups(grid:&mut Grid<String>, groups:&mut Vec<Grouping>, rng:&mut ThreadRng){
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
fn print_grid(grid:&Grid<String>, title:&str) {
    println!("{}", title);
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

/// # print_groupings(groupings, title)
/// 
/// This function prints the specified list of groupings to the console for debugging purposes.
fn print_groupings(groupings:&Vec<Grouping>, title: &str) {
    println!("{}", title);
    for group in groupings {
        println!("{}", group.to_string());
    }//end printing out each grouping
}//end print_grouping

/// # create_empty_grid()
/// 
/// This function creates an empty grid of the specified dimensions, filled with the string "empty".
fn create_empty_grid(rows:usize, cols:usize) -> Grid<String> {
    let mut empty = Grid::new(rows, cols);
    empty.fill("empty".to_string());
    return empty;
}//end createEmptyGrid
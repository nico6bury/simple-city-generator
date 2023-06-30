use fltk::app::App;
use fltk_theme::ThemeType;
use fltk_theme::WidgetTheme;
use grid::Grid;
use grouping::GroupInstance;
use gui::GUI;
use gui::MenuChoice;
use rand::Rng;
mod grouping;
use grouping::Coord;
use grouping::Grouping;
use rand::rngs::ThreadRng;
mod gui;

fn main() {
    // create random number generator for whole program
    let mut rng = rand::thread_rng();
    // create our empty grid
    let mut city_grid: Grid<GroupInstance>;
    // create application object
    let app = App::default();
    // set app theme
    // let w_theme = WidgetTheme::new(ThemeType::AquaClassic);
    // w_theme.apply();
    
    // set up gui
    let mut gui = GUI::default(&app);
    gui.initialize_top_menu();
    gui.initialize_settings();
    
    // show the gui
    gui.show();
    while app.wait() {
        if let Some(val) = gui.msg_receiver.recv() {
            match val {
                MenuChoice::SetColor => {
                    let dist_index_option = gui.choose_district();
                    if dist_index_option.is_some() {
                        let dist_index = dist_index_option.unwrap();
                        // get a color from user
                        gui.show_message(&format!("Please select a color for district {}", gui.districts.get(dist_index).unwrap().name));
                        let gui_color_result = gui.get_color();
                        if gui_color_result.is_some() {
                            gui.districts.get_mut(dist_index).unwrap().rgb_color = gui_color_result.unwrap();
                            gui.update_district_list_buf();
                        }//end if we got a color to use
                    }//end if we got something
                },
                MenuChoice::AddDistrict => {
                    let new_dist_name = gui.get_new_district_name();
                    if new_dist_name.is_some() {
                        let new_district = Grouping::new(new_dist_name.unwrap());
                        println!("Adding district {}", new_district.name);
                        gui.districts.push(new_district);
                        gui.update_district_list_buf();
                    }//end if we got a name for a new district
                },
                MenuChoice::RemoveDistrict => {
                    // figure out what to remove
                    let district_index_to_remove = gui.choose_district();
                    // remove the specified district
                    if district_index_to_remove.is_some() {
                        let removed = gui.districts.remove(district_index_to_remove.unwrap());
                        println!("Removed district {}", removed.name);
                        gui.update_district_list_buf();
                    }//end if we can remove one
                },
                MenuChoice::GenerateDistricts => {
                    // figure out grid row and column width to make new grid
                    let grid_dims = gui.get_districts_dims();
                    city_grid = create_empty_grid(grid_dims.0, grid_dims.1);

                    // reset district locations
                    gui.clear_district_locations();

                    // add group starts in random spots
                    println!("Starting grid priming");
                    prime_grid_with_groups(&mut city_grid, &mut gui.districts, &mut rng, 10, 10);
                    println!("Grid is primed.");
                    
                    // advance groups until enclosed
                    let mut all_enclosed = false;
                    println!("Starting grid generation");
                    while !all_enclosed {
                        let num_enclosed = advance_group_expansion(&mut city_grid, &mut gui.districts, &mut rng, 10, 10);
                        println!("{} groups are fully enclosed", &num_enclosed);
                        all_enclosed = num_enclosed.eq(&gui.districts.len());
                    }//end looping while some groupings are still able to expand
                    println!("Finished generating grid");

                    // display the new grid stuff
                    gui.update_grid(&city_grid);
                },
                _ => {println!("Unhandled Message");}
            }//end matching message values
        }//end if we received a message from receiver
    }//end application loop
}//end main function

/// # advance_group_expansion(grid, groups)
/// 
/// For each group, an adjacent, unclaimed tile will be claimed. If no tiles can be claimed, the group will not expand.
/// 
/// ## Parameters
/// inner_rows and inner_cols refer to the number of rows and columns to initialize the grid in each GroupInstance with
/// groups is the list of possible groups or categories to choose from
/// the grid represents which spots have been claimed by which groups
/// 
/// ## Return
/// This function returns the number of groups which could not be expanded because they were completely enclosed
fn advance_group_expansion(grid:&mut Grid<GroupInstance>, groups:&mut Vec<Grouping>, rng:&mut ThreadRng, inner_rows:usize, inner_cols:usize) -> usize {
    // counter to keep track of fully enclosed groups
    let mut num_enclosed: usize = 0;
    // start looping through the groups to advance
    for group in groups {
        // find the coords which are adjacent to group and unclaimed by any other group
        let adjacent_coords = group.get_adjacent_coords(grid.rows() - 1, grid.cols() - 1, false);
        let mut open_coords = Vec::new();
        for coord in adjacent_coords {
            let this_group = &grid.get(coord.row, coord.col).unwrap().group;
            if this_group.is_none() {
                open_coords.push(coord);
            }//end if the coord is still unclaimed
        }//end checking each coord in adjacent_coords to add to open_coords

        // don't try to advance if there's no valid open coords
        if open_coords.len() == 0 {
            num_enclosed += 1;
            continue;
        }//end if we have an enclosed district
        // pick one of the open_coords
        let coord_to_use = weighted_coord_rng(rng, &mut open_coords, group).to_owned();
        // update group name in grid
        let grid_spot = grid.get_mut(coord_to_use.row, coord_to_use.col).unwrap();
        // update grouping locations
        group.locations.push(coord_to_use.clone());
        // update grid ref
        *grid_spot = GroupInstance::new(group.clone(), coord_to_use.clone(), inner_rows, inner_cols);
    }//end looping over each group to advance
    return num_enclosed;
}//end advance_group_expansion(grid, groups)

/// # weighted_coord_rng
/// 
/// Does some weird number manipulation to prefer lower distances in random coord picking.
/// Uses adapted algorithm from http://stackoverflow.com/questions/1761626/weighted-random-numbers/1761646#1761646
/// 
/// This function can handle coords having only one element, but don't call it with coords being empty.
/// 
/// ## Previous Issue: Potential Panic
/// The function works by summing up the dist_from_center of each coord. If for some reason, this sum is 0, then the function will panic, saying something about rng not working because something something range bounds. Basically it doesn't like being called with a range of "0..0". This *shouldn't* happen anymore, but if panics like that start cropping up, then take a look at this function or the dist_from_center function in grouping.rs.
fn weighted_coord_rng<'a>(rng: & mut ThreadRng, coords:&'a Vec<Coord>, grouping:&'a Grouping) -> &'a Coord  {
    // edge case for only one coord
    if coords.len() == 1 {
        return coords.first().unwrap();
    }//end if we just have one choice
    // figure out the sum of distances
    let mut sum_of_weight = 0;
    for coord in coords {
        sum_of_weight += grouping.dist_from_center(coord).ceil() as i32;
    }//end summing up distances from center
    // do the generation ???
    let mut rnd_num = rng.gen_range(0..sum_of_weight);
    for coord in coords {
        let this_weight = grouping.dist_from_center(coord).ceil() as i32;
        if rnd_num < this_weight {
            return coord;
        }//end if we have a winner
        rnd_num -= this_weight;
    }//end looping over choice coords

    // should never make it here, but use unweighted generation if weighted doesn't work for some reason
    return coords.get(rng.gen_range(0..coords.len())).unwrap();
}//end weighted_coord_rng

/// # prime_grid_with_groups()
/// 
/// Adds single instance of each group in random spots in the grid.
/// inner_rows and inner_cols refer to the number of rows and columns that the grouped instance should have.
fn prime_grid_with_groups(grid:&mut Grid<GroupInstance>, groups:&mut Vec<Grouping>, rng:&mut ThreadRng, inner_rows:usize, inner_cols:usize){
    // start looping through groups to actually do stuff
    for group in groups {
        loop {
            // generate random location
            let row = rng.gen_range(0..grid.rows());
            let col = rng.gen_range(0..grid.cols());
            // check that we're not overlapping
            let this_group = &grid.get(row, col).unwrap().group;
            if this_group.is_none() {
                // actually put the group in
                let spot = grid.get_mut(row, col).unwrap();
                // get the Coord for this new group instance
                let this_coord = Coord::new(row, col);
                // update the grouping
                group.locations.push(this_coord.clone());
                // put the right references into this GroupInstance
                *spot = GroupInstance::new(group.clone(), this_coord.clone(), inner_rows, inner_cols);
                break;
            }//end if we can continue
            else {continue;}
        }//end while we need to do check to not overwrite other group
    }//end generating something for each group
}//end prime_grid_with_groups

/// # print_grid()
/// 
/// This function prints the specified grid to the console for debugging purposes.
#[allow(dead_code)]
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
#[allow(dead_code)]
fn print_groupings(groupings:&Vec<Grouping>, title: &str) {
    println!("{}", title);
    for group in groupings {
        println!("{}", group.to_string());
    }//end printing out each grouping
}//end print_grouping

/// # create_empty_grid()
/// 
/// This function creates an empty grid of the specified dimensions, filled with the string "empty".
fn create_empty_grid(rows:usize, cols:usize) -> Grid<GroupInstance> {
    let mut empty = Grid::new(rows, cols);
    empty.fill(GroupInstance::default());
    return empty;
}//end createEmptyGrid
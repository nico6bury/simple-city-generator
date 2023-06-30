use fltk::app::App;
use fltk_theme::ThemeType;
use fltk_theme::WidgetTheme;
use grid::Grid;
use grouping::BuildingType;
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
        if let Some(val) = gui.menu_msg_receiver.recv() {
            match val.as_str() {
                "MenuChoice::SetColor" => {
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
                "MenuChoice::AddDistrict" => {
                    let new_dist_name = gui.get_new_district_name();
                    if new_dist_name.is_some() {
                        let new_district = Grouping::new(new_dist_name.unwrap());
                        println!("Adding district {}", new_district.name);
                        gui.districts.push(new_district);
                        gui.update_district_list_buf();
                    }//end if we got a name for a new district
                },
                "MenuChoice::RemoveDistrict" => {
                    // figure out what to remove
                    let district_index_to_remove = gui.choose_district();
                    // remove the specified district
                    if district_index_to_remove.is_some() {
                        let removed = gui.districts.remove(district_index_to_remove.unwrap());
                        println!("Removed district {}", removed.name);
                        gui.update_district_list_buf();
                    }//end if we can remove one
                },
                "MenuChoice::GenerateDistricts" => {
                    // figure out district row and column width to make new grid
                    let distr_dims = gui.get_districts_dims();
                    city_grid = create_empty_grid(distr_dims.0, distr_dims.1);
                    // figure out neighborhood row and column width for inner grids
                    let neigh_dims = gui.get_neighborhood_dims();

                    // reset district locations
                    gui.clear_district_locations();

                    // add group starts in random spots
                    println!("\nStarting grid priming");
                    prime_grid_with_groups(&mut city_grid, &mut gui.districts, &mut rng, neigh_dims.0, neigh_dims.1);
                    println!("\nGrid is primed.");
                    
                    // advance groups until enclosed
                    let mut all_enclosed = false;
                    println!("\nStarting grid generation");
                    while !all_enclosed {
                        let num_enclosed = advance_group_expansion(&mut city_grid, &mut gui.districts, &mut rng, neigh_dims.0, neigh_dims.1);
                        println!("{} groups are fully enclosed", &num_enclosed);
                        all_enclosed = num_enclosed.eq(&gui.districts.len());
                    }//end looping while some groupings are still able to expand
                    println!("\nFinished generating grid");

                    // display the new grid stuff
                    gui.update_grid(&city_grid);

                    // generate all neighborhoods
                    println!("\nStarting neighborhood generation");
                    for row in 0..city_grid.rows() {
                        for col in 0..city_grid.cols() {
                            let this_instance = city_grid.get_mut(row, col).expect("valid index");
                            generate_neighborhood(this_instance, &mut rng);
                        }//end looping over cols in city grid
                    }//end looping over rows in city grid
                    println!("Finished neighborhood generation\n");

                    // TODO: Make it so that neighborhoods can be displayed
                },
                _ => {
                    if val.contains(',') {
                        let coord_vals: Vec<&str> = val.split(',').collect();
                        if coord_vals.len() == 2 {
                            // get our row and col index parsed
                            let row_idx:usize = coord_vals.get(0).unwrap().parse().unwrap();
                            let col_idx:usize = coord_vals.get(1).unwrap().parse().unwrap();
                            // print out the row-col pair for all to see
                            println!("Received message asking after neighborhood at row {} and column {}", row_idx + 1, col_idx + 1);
                            // TODO: Send Message to GUI to display neighborhood at coordinate
                        }//end found a coordinate pair
                    }//end if we have a comma-separated value
                    else {println!("Unhandled message!!\n")}
                }//end if we have an irregular message
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
        let coord_to_use = weighted_coord_rng_districts(rng, &mut open_coords, group).to_owned();
        // update group name in grid
        let grid_spot = grid.get_mut(coord_to_use.row, coord_to_use.col).unwrap();
        // update grouping locations
        group.locations.push(coord_to_use.clone());
        // update grid ref
        *grid_spot = GroupInstance::new(group.clone(), coord_to_use.clone(), inner_rows, inner_cols);
    }//end looping over each group to advance
    return num_enclosed;
}//end advance_group_expansion(grid, groups)

/// # weighted_coord_rng_districts
/// 
/// Does some weird number manipulation to prefer lower distances in random coord picking.
/// Uses adapted algorithm from http://stackoverflow.com/questions/1761626/weighted-random-numbers/1761646#1761646
/// 
/// This function can handle coords having only one element, but don't call it with coords being empty.
/// 
/// ## Previous Issue: Potential Panic
/// The function works by summing up the dist_from_center of each coord. If for some reason, this sum is 0, then the function will panic, saying something about rng not working because something something range bounds. Basically it doesn't like being called with a range of "0..0". This *shouldn't* happen anymore, but if panics like that start cropping up, then take a look at this function or the dist_from_center function in grouping.rs.
fn weighted_coord_rng_districts<'a>(rng: & mut ThreadRng, coords:&'a Vec<Coord>, grouping:&'a Grouping) -> &'a Coord  {
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
}//end weighted_coord_rng_districts

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

/// # generate_neighborhood
/// 
/// Generates a neighborhood grid inside nhood's sub_grid field.
/// This method will use the rows and columns of the provided nhood object.
/// The rng parameter is used for random number generation.
fn generate_neighborhood(nhood:&mut GroupInstance, rng:&mut ThreadRng) {
    // save some handy reference variables for later
    let rows = nhood.sub_grid.rows();
    let cols = nhood.sub_grid.cols();
    
    // add roads to our neighborhood
    let num_colors = add_roads_to_neighborhood(nhood, rng);

    // figure out number of color options to use
    let color_options = gen_nhood_colors(rng, num_colors);

    // loop through the whole grid
    for row in 0..rows {
        for col in 0..cols {
            // pull out the building we want to edit
            let this_build = nhood.sub_grid.get_mut(row, col).expect("Index should have been safe.");
            
            // make sure we aren't overwriting a road
            if this_build.build_type.eq(&BuildingType::Road) {continue;}

            // generate new type and color for building
            let (build_type, color) = gen_build_type_color(rng, &color_options);
            this_build.build_type = build_type;
            this_build.rgb_color = color;
        }//end looping through columns
    }//end looping through rows
}//end generate_neighborhood(nhood, rng)

/// # add_roads_to_neighborhood(nhood, rng)
/// 
/// This function could be seen as a helper function for generate_neighborhood().
/// It will generate roads and place them in the nhood parameter.
/// 
/// Returns the recommended number of colors to use
fn add_roads_to_neighborhood(nhood:&mut GroupInstance, rng:&mut ThreadRng) -> usize{
    // save some handy reference variables for later
    let rows = nhood.sub_grid.rows();
    let cols = nhood.sub_grid.cols();
    
    // figure out number of roads to slam in there
    let num_roads_low_bound = ((rows + cols) as f32 * 0.15).ceil() as usize;
    let num_roads_upp_bound = ((rows + cols) as f32 * 0.4).ceil() as usize;
    let num_roads_total = rng.gen_range(num_roads_low_bound..num_roads_upp_bound);
    let num_roads_horizontal = rng.gen_range(1.min(num_roads_total / 2)..num_roads_total.min((num_roads_total as f32 * 0.7).ceil() as usize));
    let num_roads_vertical = num_roads_total - num_roads_horizontal;

    // determine number of colors from roads
    let num_colors = 3.max(num_roads_upp_bound - num_roads_total);
    
    // slap some horizontal roads in there
    let mut roads_hor_idxs = Vec::new();
    while roads_hor_idxs.len() < num_roads_horizontal.min(rows) {
        let road_hor_idx = rng.gen_range(0..rows);
        if !roads_hor_idxs.contains(&road_hor_idx) {
            roads_hor_idxs.push(road_hor_idx);
        }//end if we haven't already generated this index
    }//end looping while we can fit some more horizontal roads in there

    // slap some vertical roads in there
    let mut roads_ver_idxs = Vec::new();
    while roads_ver_idxs.len() < num_roads_vertical.min(cols) {
        let road_ver_idx = rng.gen_range(0..cols);
        if !roads_ver_idxs.contains(&road_ver_idx) {
            roads_ver_idxs.push(road_ver_idx);
        }//end if we haven't already generated this index
    }//end looping while we can fit some more vertical roads int there

    let road_color:(u8,u8,u8) = (55,55,55);

    // actually edit nhood with horizontal roads
    for row_idx in roads_hor_idxs {
        for col_idx in 0..cols {
            let this_building = nhood.sub_grid.get_mut(row_idx, col_idx).expect("Those indices seemed pretty valid to me... Should be in bounds and everything.");
            // set type to road and color the roads
            this_building.build_type = BuildingType::Road;
            this_building.rgb_color = road_color;
        }//end looping over each column on our way horizontal
    }//end looping over each horizontal road index to add

    // actually edit nhood with vertical roads
    for col_idx in roads_ver_idxs {
        for row_idx in 0..rows {
            let this_building = nhood.sub_grid.get_mut(row_idx, col_idx).expect("Those indices seemed pretty valid to me... Should be in bounds and everything.");
            // set type to road and color the roads
            this_building.build_type = BuildingType::Road;
            this_building.rgb_color = road_color;
        }//end looping over reach row on our way vertical
    }//end looping over each vertical road index

    return num_colors;
}//end add_roads_to_neighborhood(nhood, rng)

/// # get_nhood_colors(rng, num_colors)
/// 
/// This function could be seen as a helper function for generate_neighborhood().
/// It will generate a random list of rgb colors, with the number of colors depending on the input number of roads. Generally, the idea is that if you have a relative surplus of roads, then you want relatively fewer colors, and vice versa.
/// 
/// Returns a vector of (u8,u8,u8), representing rgb values.
fn gen_nhood_colors(rng: & mut ThreadRng, num_colors:usize) -> Vec<(u8, u8, u8)> {
    // get our vector of colors
    let mut color_options:Vec<(u8,u8,u8)> = Vec::new();

    let px_num = 42.max(num_colors * 2);
    let px_interval = 255 / px_num;
    while color_options.len() < num_colors {
        // generate an rgb value somewhat randomly
        let r = (rng.gen_range(1..(px_num - 1)) * px_interval) as u8;
        let g = (rng.gen_range(1..(px_num - 1)) * px_interval) as u8;
        let b = (rng.gen_range(1..(px_num - 1)) * px_interval) as u8;
        let rgb = (r,g,b);

        if !color_options.contains(&rgb) {
            color_options.push(rgb);
        }//end if we found a new color value
    }//end looping while we should still fill our list

    return color_options;
}//end gen_nhood_colors()

/// # gen_build_type_color(rng, colors)
/// 
/// This function could be seen as a helper function for generate_neighborhoods().
/// It will randomly generate a building type and a color. That's it, just useful to keep generate_neighborhoods a little bit cleaner.
/// 
/// returns a tuple containing (BuildingType, rgb color as (u8,u8,u8))
fn gen_build_type_color(rng:&mut ThreadRng, colors:&Vec<(u8,u8,u8)>) -> (BuildingType, (u8, u8, u8)) {
    let build_type_index = rng.gen_range(0..10);
    let build_type = match build_type_index {
        0 => BuildingType::Road,
        1..=5 => BuildingType::Residence,
        6..=8 => BuildingType::Shop,
        _ => BuildingType::Empty,
    };
    let color = colors.get(rng.gen_range(0..colors.len())).expect("Proper indexing").to_owned();
    return (build_type, color);
}//end gen_build_type_color(rng, colors)

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

/// # print_neighborhood(nhood, title)
/// 
/// 
#[allow(dead_code)]
fn print_neighborhood(nhood:&mut GroupInstance, title: &str) {
    println!("{}",title);
    for row in 0..nhood.sub_grid.rows() {
        for col in 0..nhood.sub_grid.cols() {
            print!("{}\t", nhood.sub_grid.get(row, col).unwrap().build_type);
        }//end looping over col indices
        print!("\n");
    }//end looping over row indices
    println!("\n");
}//end print_neighborhood

/// # create_empty_grid()
/// 
/// This function creates an empty grid of the specified dimensions, filled with the string "empty".
fn create_empty_grid(rows:usize, cols:usize) -> Grid<GroupInstance> {
    let mut empty = Grid::new(rows, cols);
    empty.fill(GroupInstance::default());
    return empty;
}//end createEmptyGrid
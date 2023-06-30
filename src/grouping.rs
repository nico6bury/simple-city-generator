
/// # Coord
/// 
/// just a shorthand for a tuple containing an row and col index
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
	/// # row
	/// 
	/// The row index
	pub row: usize,
	/// # col
	/// 
	/// The column index
	pub col: usize,
}//end struct Coord

#[allow(dead_code)]
impl Coord {
	/// # new(row, col)
	/// 
	/// This method initializes a coord with its row and column indices.
	pub fn new(row:usize, col:usize) -> Coord {
		Coord {
			row,
			col,
		}//end struct construction
	}//end new()
	
	/// # to_string(&self)
	/// 
	/// returns a string with the labelled row and column index
	pub fn to_string(&self) -> String {
		format!("row: {}, col: {}", self.row, self.col)
	}//end to_string()
}//end impl for Coord

/// # Grouping
/// 
/// A struct to keep track of the instances a group is located within a grid
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Grouping {
	/// # group
	/// 
	/// the name of this grouping
	pub name: String,
	/// # location
	/// 
	/// list of coordinates where this group is located in a grid
	pub locations: Vec<Coord>,
	/// # rgb_color
	/// 
	/// the color to display for the group, in 3-number rgb values
	pub rgb_color: (u8,u8,u8),
}//end struct grouping

#[allow(dead_code)]
impl Grouping {
	/// # new(name)
	/// 
	/// Creates a new grouping with the specified name.
	pub fn new(name:String) -> Grouping {
		Grouping {
			name,
			locations: Vec::new(),
			rgb_color: (0,0,0),
		}//end struct construction
	}//end new()

	/// # default()
	/// 
	/// Creates a new grouping with name "default"
	pub fn default() -> Grouping {
		Grouping {
			name: "default".to_string(),
			locations: Vec::new(),
			rgb_color: (0,0,0),
		}//end struct construction
	}//end default()

	/// # to_string(&self)
	/// 
	/// This method creates a string consisting of this grouping's name and all locations, complete with labelled indices.
	pub fn to_string(&self) -> String {
		let mut result = format!("Group Printout:\nName: {}\n", self.name);
		for location in &self.locations {
			result = format!("{}{}\n", result, location.to_string());
		}//end adding location info to string
		return result;
	}//end to_string(&self)

	/// # get_adjacent_coords(&self, max_row, max_col)
	/// 
	/// This function generates a list of coordinates that are adjacent to this grouping. The maximum row and column index are required in parameters. 
	/// This function will automatically exclude coordinates that are already apart of this grouping or that would be out of bounds.
	pub fn get_adjacent_coords(&self, max_row: usize, max_col: usize, allow_diagonal: bool) -> Vec<Coord> {
		let mut adjacents = Vec::new();
		for location in &self.locations {
			// handy references to make the code more concise
			let row = location.row;
			let col = location.col;
			// top left
			if row > 0 && col > 0 && allow_diagonal {
				let top_left = Coord::new(row - 1, col - 1);
				if !self.locations.contains(&top_left) && !adjacents.contains(&top_left) {
					adjacents.push(top_left);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// top
			if row > 0 {
				let top = Coord::new(row - 1,col);
				if !self.locations.contains(&top) && !adjacents.contains(&top) {
					adjacents.push(top);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// top right
			if row > 0 && col < max_col && allow_diagonal {
				let top_right = Coord::new(row - 1,col + 1);
				if !self.locations.contains(&top_right) && !adjacents.contains(&top_right) {
					adjacents.push(top_right);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// mid left
			if col > 0 {
				let mid_left = Coord::new(row, col - 1);
				if !self.locations.contains(&mid_left) && !adjacents.contains(&mid_left) {
					adjacents.push(mid_left);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// mid right
			if col < max_col {
				let mid_right = Coord::new(row, col + 1);
				if !self.locations.contains(&mid_right) && !adjacents.contains(&mid_right) {
					adjacents.push(mid_right);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// bottom left
			if row < max_row && col > 0 && allow_diagonal {
				let bot_left = Coord::new(row + 1, col - 1);
				if !self.locations.contains(&bot_left) && !adjacents.contains(&bot_left) {
					adjacents.push(bot_left);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// bottom
			if row < max_row {
				let bot = Coord::new(row + 1, col);
				if !self.locations.contains(&bot) && !adjacents.contains(&bot) {
					adjacents.push(bot);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// bottom right
			if row < max_row && col < max_col && allow_diagonal {
				let bot_right = Coord::new(row + 1, col + 1);
				if !self.locations.contains(&bot_right) && !adjacents.contains(&bot_right) {
					adjacents.push(bot_right);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
		}//end looping over locations
		return adjacents;
	}//end get_adjacent_coords()

	/// # dist_from_center(&self, coord)
	/// 
	/// Returns the distance as a float from the first location added to this grouping.
	pub fn dist_from_center(&self, coord: &Coord) -> f32 {
		// pull out first location as a variable for easy reference
		let first = self.locations.first().unwrap();
		// do a little pythag theorem
		let x_diff = f32::abs((first.row as f32 - coord.row as f32) as f32);
		let y_diff = f32::abs((first.col as f32 - coord.col as f32) as f32);
		let x_diff_squared = x_diff * x_diff;
		let y_diff_squared = y_diff * y_diff;
		let x_y_squared_sum = x_diff_squared + y_diff_squared;
		let distance = f32::sqrt(x_y_squared_sum as f32);
		return distance;
	}//end dist_from_center(&self, coord)
}//end impl for Grouping

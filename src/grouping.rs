
/// # Coord
/// 
/// just a shorthand for a tuple containing an row and col index
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
	/// # row
	/// 
	/// The row index
	row: usize,
	/// # col
	/// 
	/// The column index
	col: usize,
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grouping {
	/// # group
	/// 
	/// the name of this grouping
	pub name: String,
	/// # location
	/// 
	/// list of coordinates where this group is located in a grid
	pub locations: Vec<Coord>,
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
		}//end struct construction
	}//end new()

	/// # default()
	/// 
	/// Creates a new grouping with name "default"
	pub fn default() -> Grouping {
		Grouping {
			name: "default".to_string(),
			locations: Vec::new(),
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
	pub fn get_adjacent_coords(&self, max_row: usize, max_col: usize) -> Vec<Coord> {
		let mut adjacents = Vec::new();
		for location in &self.locations {
			// handy references to make the code more concise
			let row = location.row;
			let col = location.col;
			// top left
			if row > 0 && col > 0 {
				let top_left = Coord::new(row - 1, col - 1);
				if !self.locations.contains(&top_left) && !adjacents.contains(&top_left) {
					adjacents.push(top_left);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// top
			if row > 0 {
				let top = Coord::new(row,col);
				if !self.locations.contains(&top) && !adjacents.contains(&top) {
					adjacents.push(top);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// top right
			if row > 0 && col < max_col {
				let top_right = Coord::new(row + 1,col - 1);
				if !self.locations.contains(&top_right) && !adjacents.contains(&top_right) {
					adjacents.push(top_right);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// mid left
			if col > 0 {
				let mid_left = Coord::new(row - 1, col);
				if !self.locations.contains(&mid_left) && !adjacents.contains(&mid_left) {
					adjacents.push(mid_left);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// mid right
			if col < max_col {
				let mid_right = Coord::new(row + 1, col);
				if !self.locations.contains(&mid_right) && !adjacents.contains(&mid_right) {
					adjacents.push(mid_right);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// bottom left
			if row < max_row && col > 0 {
				let bot_left = Coord::new(row - 1, col + 1);
				if !self.locations.contains(&bot_left) && !adjacents.contains(&bot_left) {
					adjacents.push(bot_left);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// bottom
			if row < max_row {
				let bot = Coord::new(row, col + 1);
				if !self.locations.contains(&bot) && !adjacents.contains(&bot) {
					adjacents.push(bot);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
			// bottom right
			if row < max_row && col < max_col {
				let bot_right = Coord::new(row + 1, col + 1);
				if !self.locations.contains(&bot_right) && !adjacents.contains(&bot_right) {
					adjacents.push(bot_right);
				}//end if we don't already have this adjacency
			}//end if this coordinate is in bounds
		}//end looping over locations
		return adjacents;
	}//end get_adjacent_coords()
}//end impl for Grouping

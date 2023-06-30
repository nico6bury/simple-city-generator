use grid::Grid;


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
			name: "empty".to_string(),
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

	/// # with_color(self, color)
	/// 
	/// sets rgb color without needing a separate assignment
	pub fn with_color(&mut self, color:(u8,u8,u8)) -> Grouping  {
		self.rgb_color = color;
		self.to_owned()
	}//end with_color(self, color)

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

#[derive(Clone)]
pub struct GroupInstance {
	pub group:Option<Grouping>,
	pub coord:Option<Coord>,
	pub sub_grid: Grid<Building>,
}

impl Default for GroupInstance {
	/// # default()
	/// 
	/// Sets the group and coord references as None without really setting up the sub_grid yet.
	fn default() -> GroupInstance {
		GroupInstance {
			group: None,
			coord: None,
			sub_grid: Grid::new(1,1),
		}//end struct construction
	}//end default(group, coord)
}//end Default impl for GroupInstance

#[allow(dead_code)]
impl GroupInstance {
	/// # initialize_sub_grid(self, rows, cols)
	/// 
	/// Sets grid to specified size and filles with default Buildings
	pub fn initialize_sub_grid(&mut self, rows:usize, cols:usize) {
		self.sub_grid = Grid::new(rows, cols);
		self.sub_grid.fill(Building::default());
	}//end initialize_sub_grid(self, rows, cols)

	/// # new(group, coord, rows, cols)
	/// 
	/// Sets references for grouping and coord, while also initializing sub_grid as in initialize_sub_grid() function
	pub fn new(group:Grouping, coord:Coord, rows:usize, cols:usize) -> GroupInstance {
		let mut temp_grid = Grid::new(rows, cols);
		temp_grid.fill(Building::default());

		GroupInstance {
			group: Some(group),
			coord: Some(coord),
			sub_grid: temp_grid,
		}//end struct constructions
	}//end new(group, coord, rows, cols)
}//end GroupInstance

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuildingType {
	/// building type hasn't been determined yet
	Empty,
	/// road, long and narrow, max width 1
	Road,
	/// house where people live
	Residence,
	/// place where people go to buy things
	Shop,
}//end enum BuildingType

impl Default for BuildingType {
    fn default() -> Self {
        BuildingType::Empty
    }//end default()
}//end enum BuildingType

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Building {
	pub build_type: BuildingType,
	pub rgb_color: (u8, u8, u8)
}//end struct Building

impl Building {
	
}//end impl for Building

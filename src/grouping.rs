
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
}//end impl for Grouping

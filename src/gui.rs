use fltk::app;
use fltk::app::App;
use fltk::app::Receiver;
use fltk::app::Sender;
use fltk::button::Button;
use fltk::dialog;
use fltk::enums::Align;
use fltk::enums::Color;
use fltk::enums::FrameType;
use fltk::enums::Shortcut;
use fltk::group;
use fltk::group::Flex;
use fltk::group::Group;
use fltk::group::Tabs;
use fltk::input::IntInput;
use fltk::menu;
use fltk::menu::SysMenuBar;
use fltk::prelude::DisplayExt;
use fltk::prelude::GroupExt;
use fltk::prelude::InputExt;
use fltk::prelude::MenuExt;
use fltk::prelude::WidgetBase;
use fltk::prelude::WidgetExt;
use fltk::text::TextBuffer;
use fltk::text::TextDisplay;
use fltk::window::Window;
use fltk_theme::widget_themes;
use grid::Grid;

use crate::grouping::GroupInstance;
use crate::grouping::Grouping;

#[allow(dead_code)]
#[derive(Clone)]
pub enum MenuChoice {
	Choice1,
	Choice2,
	Resize,
	SetColor,
	AddDistrict,
	RemoveDistrict,
	GenerateDistricts,
}//end enum MenuChoice

pub struct GUI<'a> {
	/// reference to application object that everything fits inside
	pub application:&'a App,
	/// the main window of the application
	pub main_window:Window,
	/// send messages for menu events
	pub menu_msg_sender:Sender<String>,
	// receive messages for menu events
	pub menu_msg_receiver:Receiver<String>,
	/// the menu bar at the top
	pub top_menu:SysMenuBar,
	/// the struct handling the 2d array of buttons for district representation
	pub grid_buttons:Grid<Button>,
	/// the struct handling the grid of buttons for districts
	pub grid_flex: FlexGrid,
	/// the struct handling the grid of buttons for neighborhoods
	pub neighborhood_flex: FlexGrid,
	/// group holding the various tabs
	pub tabs:Tabs,
	/// group holding the settings for generation
	pub settings_tab:Group,
	/// group holding the display of generated districts
	pub districts_tab:Group,
	/// group holding the display of generated districts
	pub neighborhood_tab:Group,
	/// the list of groupings that we'll generate districts from, each grouping is a district
	pub districts:Vec<Grouping>,
	/// The text buffer for displaying our list of the districts
	pub districts_list_buffer:TextBuffer,
	/// the input for number of rows of districts to generate
	districts_rows_input:IntInput,
	/// The input for number of columns of districts to generate
	districts_cols_input:IntInput,
	/// The input for number of rows of neighborhoods to generate
	neighborhood_rows_input:IntInput,
	/// The input for number of columsn of neighborhoods to generate
	neighborhood_cols_input:IntInput,
}//end struct gui

fn get_default_win_width() -> i32 {900}
fn get_default_win_height() -> i32 {480}
fn get_default_menu_height() -> i32 {20}
fn get_default_tab_padding() -> i32 {20}
fn get_default_grid_width() -> i32 {get_default_win_width()}
fn get_default_grid_height() -> i32 {get_default_win_height()-get_default_menu_height() - get_default_tab_padding()}
fn get_max_grid_button_width() -> i32 {30}
fn get_max_grid_button_height() -> i32 {15}
fn get_max_luminance_for_white_label() -> f32 {100.0}

impl GUI<'_> {
	/// # default()
	/// 
	/// 
	pub fn default<'a>(application:&'a App) -> GUI<'a> {
		let (s1, r1) = app::channel();
		let mut gui = GUI {
			application,
			main_window: Window::default(),
			menu_msg_sender: s1,
			menu_msg_receiver: r1,
			top_menu: SysMenuBar::default(),
			grid_buttons: Grid::new(0,0),
			grid_flex: FlexGrid::default(),
			neighborhood_flex: FlexGrid::default(),
			tabs: Tabs::default(),
			settings_tab: Group::default(),
			districts_tab: Group::default(),
			neighborhood_tab: Group::default(),
			districts: Vec::new(),
			districts_list_buffer: TextBuffer::default(),
			districts_rows_input: IntInput::default(),
			districts_cols_input: IntInput::default(),
			neighborhood_rows_input: IntInput::default(),
			neighborhood_cols_input: IntInput::default(),
		};//end struct construction
		gui.set_default_properties();
		return gui;
	}//end default()
	
	/// # set_default_properties
	/// 
	/// 
	pub fn set_default_properties(&mut self) {
		// main window settings
		self.main_window = self.main_window.clone()
			.with_size(get_default_win_width(), get_default_win_height())
			.with_label("CIS 536 City Generator");
		self.main_window.make_resizable(true);
		self.main_window.end();

		// set default groupings
		self.districts.push(Grouping::new("slum".to_string()).with_color((222,42,195)));
		self.districts.push(Grouping::new("suburb".to_string()).with_color((114,222,42)));
		self.districts.push(Grouping::new("adventuring".to_string()).with_color((227,0,0)));
		self.districts.push(Grouping::new("financial".to_string()).with_color((255,250,105)));
		self.districts.push(Grouping::new("business".to_string()).with_color((74,132,232)));

		// top menu settings
		self.top_menu = self.top_menu.clone()
			.with_size(get_default_win_width(), get_default_menu_height());
		self.top_menu.set_frame(FrameType::FlatBox);
		// actually make the menu show up
		self.main_window.add(&self.top_menu);

		// tabs settings
		self.tabs = Tabs::new(0, get_default_menu_height(), get_default_win_width(), get_default_win_height(), None);
		self.tabs.auto_layout();
		self.tabs.end();
		self.main_window.add(&self.tabs);

		// settings tab
		self.settings_tab = Group::new(0, self.tabs.y() + get_default_tab_padding(), self.tabs.width(), self.tabs.height(), "Settings");
		self.settings_tab.end();
		self.tabs.add(&self.settings_tab);

		// district tab
		self.districts_tab = Group::default()
			.with_pos(0, self.tabs.y() + get_default_tab_padding())
			.with_size(self.tabs.width(), self.tabs.height())
			.with_label("Districts")
			.with_type(group::FlexType::Row);
		self.districts_tab.end();
		self.tabs.add(&self.districts_tab);

		// neighborhood tab
		self.neighborhood_tab = Group::default()
			.with_pos(0, self.tabs.y() + get_default_tab_padding())
			.with_size(self.tabs.width(), self.tabs.height())
			.with_label("Neighborhood");
		self.neighborhood_tab.end();
		self.neighborhood_flex = FlexGrid::default();
		self.neighborhood_tab.add(&self.neighborhood_flex.outer_flex);
		self.tabs.add(&self.neighborhood_tab);
	}//end set_default_properties
	
	/// # switch_tab(&mut self, tab_idx:i32)
	pub fn switch_tab(&mut self, tab_idx:u8) {
		let cur_vis_val = self.tabs.value();
		if cur_vis_val.is_none() {return;}
		let cur_vis = cur_vis_val.unwrap();

		match tab_idx {
			0 => {
				if cur_vis.is_same(&self.settings_tab) {return;}
				self.tabs.set_value(&self.settings_tab).expect("tabs");
			},
			1 => {
				if cur_vis.is_same(&self.districts_tab) {return;}
				self.tabs.set_value(&self.districts_tab).expect("tabs");
			},
			2 => {
				if cur_vis.is_same(&self.neighborhood_tab) {return;}
				self.tabs.set_value(&self.neighborhood_tab).expect("tabs");
			},
			_ => {
				// do nothing
			}
		}//end matching tab index
		self.settings_tab.redraw();
		self.districts_tab.redraw();
		self.neighborhood_tab.redraw();
	}//end switch_tab(&mut self, tab_idx)

	/// # initialize_top_menu
	/// 
	/// 
	pub fn initialize_top_menu(&mut self) {
		// set up all the emitters
		self.top_menu.add_emit(
			"&File/Choice1...\t",
			Shortcut::Ctrl | 'n',
			menu::MenuFlag::Normal,
			self.menu_msg_sender.clone(),
			"MenuChoice::Choice1".to_string(),
		);
		self.top_menu.add_emit(
			"&File/Choice2...\t",
			Shortcut::Ctrl | 'o',
			menu::MenuFlag::Normal,
			self.menu_msg_sender.clone(),
			"MenuChoice::Choice2".to_string(),
		);
		self.top_menu.add_emit(
			"Regen",
			Shortcut::Ctrl | 'r',
			menu::MenuFlag::Normal,
			self.menu_msg_sender.clone(),
			"MenuChoice::GenerateDistricts".to_string(),
		);
	}//end initialize_top_menu
	
	/// # update_grid(self, ext_grid)
	/// 
	/// Updates the grid view, and also initializes
	pub fn update_grid(&mut self, ext_grid:&Grid<GroupInstance>) {
		//self.grid_flex = FlexGrid::default();
		// clear previous nonsense
		if self.grid_flex.outer_flex.children() > 0 {
			self.grid_flex.clear_inner_flexes();
		}//end if we have previous stuff to take care of
		// create grid of buttons
		let mut button_grid: Grid<Button> = Grid::new(ext_grid.rows(), ext_grid.cols());
		// set size of buttons
		let button_width = get_default_grid_width() / ext_grid.cols() as i32;
		let button_height = get_default_grid_height() / ext_grid.rows() as i32;
		let show_label = button_width > get_max_grid_button_width() && button_height > get_max_grid_button_height();
		// print button size for debugging purposes
		println!("district button sizes w:{}\th:{}\tshow label:{}", button_width, button_height, show_label);
		// start looping over everything
		for row_index in 0..button_grid.rows() {
			// let mut temp_vec: Vec<Button> = Vec::new();
			for col_index in 0..button_grid.cols() {
				// reference variable for this group instance
				let this_group = ext_grid.get(row_index as usize, col_index as usize).unwrap();
				// get mutable reference for spot in grid
				let this_button_spot = button_grid.get_mut(row_index, col_index).unwrap();

				// figure out name limits
				let mut shrunk;
				if this_group.group.is_some() {
					shrunk = this_group.group.as_ref().unwrap().name.clone();
				}//end if the instance is categorized
				else { shrunk = "empty".to_string(); }
				if show_label {
					if shrunk.len() > 9 {
						shrunk = shrunk[0..9].to_string();
					}//end if we need to shrink the name
				}//end if we should show the label at al
				else { shrunk = "".to_string(); }

				// make the button, positioned correctly
				let mut new_button = Button::default()
					.with_size(button_width, button_height)
					.with_label(&shrunk);

				// set button color based on grouping
				if this_group.group.is_some() {
					let c = this_group.group.as_ref().unwrap().rgb_color;
					// new_button.set_label_color(Color::from_rgb(c.0, c.1, c.2));
					new_button.set_color(Color::from_rgb(c.0, c.1, c.2));
					// do some calculations to determine if we should change label color
					let luminance = 0.299*c.0 as f32 + 0.587*c.1 as f32 + 0.114*c.2 as f32;
					if luminance > get_max_luminance_for_white_label() {
						new_button.set_label_color(Color::Black);
					}//end if label color should be black
					else { new_button.set_label_color(Color::White); }
				}//end if this grouped instance is actually grouped
				
				// add click event/emission
				new_button.emit(self.menu_msg_sender.clone(), format!("{},{}",this_group.coord.unwrap().row,this_group.coord.unwrap().col));

				// update mutable refernce with our new button
				*this_button_spot = new_button;
			}//end converting each string into a button
		}//end going through each row

		// save our hard-earned button grid in our struct
		self.grid_buttons = button_grid;
		// make the flex grid
		self.grid_flex.initialize_flex(ext_grid.rows(), ext_grid.cols());
		self.grid_flex.fill_flex(&self.grid_buttons);
		// reposition flex grid because it likes to get lost (also screws up resizing for some ungodly reason)
		// self.grid_flex.outer_flex.resize(0 - button_width / 2, self.districts_tab.y(), self.grid_flex.outer_flex.width() + button_width / 2, self.grid_flex.outer_flex.height());
		// actually make the grid show up
		if self.districts_tab.children() < 1 {
			self.districts_tab.add(&self.grid_flex.outer_flex);
		}//end to tab if not there already
		else {
			self.districts_tab.add(&self.grid_flex.outer_flex);
			self.districts_tab.redraw();
		}
		self.grid_flex.outer_flex.recalc();
		self.grid_flex.outer_flex.redraw();
	}//end initialize_grid

	/// # initialize_setting(self)
	/// 
	/// 
	pub fn initialize_settings(&mut self) {
		// int inputs for district rows and columns
		self.districts_rows_input = IntInput::default()
			.with_size(50, 20)
			.with_pos(150, 50)
			.with_label("District Rows");
		self.districts_rows_input.set_value("10");
		self.districts_cols_input = IntInput::default()
			.with_size(50, 20)
			.right_of(&self.districts_rows_input, 135)
			.with_label("District Columns");
		self.districts_cols_input.set_value("10");

		// int inputs for neighborhood rows and columns
		self.neighborhood_rows_input = IntInput::default()
			.with_size(50, 20)
			.right_of(&self.districts_cols_input, 190)
			.with_label("Neighborhood Rows");
		self.neighborhood_rows_input.set_value("10");
		self.neighborhood_cols_input = IntInput::default()
			.with_size(50, 20)
			.right_of(&self.neighborhood_rows_input, 170)
			.with_label("Neighborhood Cols");
		self.neighborhood_cols_input.set_value("10");

		// buttons for editing districts
		let mut set_color_button = Button::default()
			.with_size(130, 30)
			.with_pos(50, 100)
			.with_label("Set Color...");
		set_color_button.emit(self.menu_msg_sender.clone(), "MenuChoice::SetColor".to_string());
		set_color_button.set_frame(widget_themes::OS_HOVERED_UP_BOX);
		let mut add_district_button = Button::default()
			.with_size(130, 30)
			.below_of(&set_color_button, 10)
			.with_label("Add District...");
		add_district_button.emit(self.menu_msg_sender.clone(), "MenuChoice::AddDistrict".to_string());
		add_district_button.set_frame(widget_themes::OS_HOVERED_UP_BOX);
		let mut remove_district_button = Button::default()
			.with_size(130, 30)
			.below_of(&add_district_button, 10)
			.with_label("Remove District...");
		remove_district_button.emit(self.menu_msg_sender.clone(), "MenuChoice::RemoveDistrict".to_string());
		remove_district_button.set_frame(widget_themes::OS_HOVERED_UP_BOX);

		// button for generating districts
		let mut gen_districts_button = Button::default()
			.with_size(150, 40)
			.below_of(&remove_district_button, 50)
			.with_label("Generate Districts");
		gen_districts_button.emit(self.menu_msg_sender.clone(), "MenuChoice::GenerateDistricts".to_string());
		gen_districts_button.set_frame(widget_themes::OS_HOVERED_UP_BOX);

		// scrollable text display for showing districts
		let mut dist_list_disp = TextDisplay::default()
			.with_size(270, 300)
			.right_of(&set_color_button, 50)
			.with_label("Districts to Generate");
		// populate district list buffer to show districts
		self.update_district_list_buf();
		// update text display with buffer
		dist_list_disp.set_buffer(self.districts_list_buffer.clone());

		// add everything to settings tab
		self.settings_tab.add(&self.districts_rows_input);
		self.settings_tab.add(&self.districts_cols_input);
		self.settings_tab.add(&self.neighborhood_rows_input);
		self.settings_tab.add(&self.neighborhood_cols_input);
		self.settings_tab.add(&set_color_button);
		self.settings_tab.add(&add_district_button);
		self.settings_tab.add(&remove_district_button);
		self.settings_tab.add(&gen_districts_button);
		self.settings_tab.add(&dist_list_disp);
	}//end initialize_settings(self)

	/// # get_districts_dims(&self)
	/// 
	/// gets the number of rows and columns for district dimensions
	pub fn get_districts_dims(&mut self) -> (usize, usize) {
		// get the raw values
		let mut rows_result: i32 = self.districts_rows_input.value().parse().expect("Should have been an int???");
		let mut cols_result: i32 = self.districts_cols_input.value().parse().expect("Should have been an int???");
		
		// do a little input handling
		rows_result = rows_result.max(self.districts.len() as i32);
		cols_result = cols_result.max(self.districts.len() as i32);

		// make sure we update our text in case we handled input
		self.districts_rows_input.set_value(&rows_result.to_string());
		self.districts_cols_input.set_value(&cols_result.to_string());

		// return dims
		(rows_result as usize, cols_result as usize)
	}//end get_districts_dims

	/// # get_neighborhood_dims(&self)
	/// 
	/// gets the number of rows and columns for neighborhood dimensions
	pub fn get_neighborhood_dims(&mut self) -> (usize, usize) {
		// get the raw values
		let mut rows_result: i32 = self.neighborhood_rows_input.value().parse().expect("Should have been an int???");
		let mut cols_result: i32 = self.neighborhood_cols_input.value().parse().expect("Should have been an int???");
		
		// do a little input handling
		rows_result = rows_result.max(3);
		cols_result = cols_result.max(3);

		// make sure we update our text in case we handled input
		self.neighborhood_rows_input.set_value(&rows_result.to_string());
		self.neighborhood_cols_input.set_value(&cols_result.to_string());

		// return dims
		(rows_result as usize, cols_result as usize)
	}//end get_neighborhood_dims

	/// # update_district_list_buf
	/// 
	/// updates the text buffer to show the list of districts
	pub fn update_district_list_buf(&mut self) {
		self.districts_list_buffer.set_text("");
		for district in &self.districts {
			let mut shrunk_name = district.name.clone();
			if shrunk_name.len() > 50 {
				shrunk_name = shrunk_name[0..50].to_string();
			}//end if we need to shrink the name
			self.districts_list_buffer.append(&format!("{},      rgb color: {},{},{}\n", shrunk_name, &district.rgb_color.0, &district.rgb_color.1, &district.rgb_color.2));
		}//end adding each district to buffer
	}//end update_district_list_buf(&mut self)

	/// # clear_district_locations(self)
	/// 
	/// clears locations in districts. best to do this whenever regenerating things
	pub fn clear_district_locations(&mut self) {
		for dist_idx in 0..self.districts.len() {
			self.districts.get_mut(dist_idx).unwrap().locations.clear();
		}//end clearing each district's locations
	}//end clear_district_locations(self)

	/// # show(self)
	/// 
	/// Simply causes the gui to become visible
	pub fn show(&mut self) {
		self.grid_flex.outer_flex.recalc();
		self.main_window.show();
	}//end show(&mut self)

	/// # get_color(&self)
	/// 
	/// optionally returns an rgb value in the form of a 3-tuple of u8 values.
	pub fn get_color(&self) -> Option<(u8,u8,u8)> {
		dialog::color_chooser("Choose Color", dialog::ColorMode::Rgb)
	}//end get_color(&self)

	/// # get_new_district_name(&self)
	/// 
	/// Opens a dialgue box and displays it to the user, prompting them to give a new district name.
	pub fn get_new_district_name(&self) -> Option<String> {
		let dialog = "Enter the name for a new district. It cannot be empty.";

		loop {
			let dialog_result = dialog::input(0, 0, dialog, "");
			if dialog_result.is_some() {
				let result = dialog_result.unwrap();
				if !result.eq("") && result.eq_ignore_ascii_case("empty") {
					dialog::message(0, 0, "District name cannot be empty :-(. Try again.");
				}//end if user had empty name
				else { return Some(result); }
			}//end if we got something
			else {return None;}
		}//end looping until we get something valid
	}//end get_new_district_name(&self)

	/// # show_message(&self, msg)
	/// 
	/// displays a simple message box with the specified message
	pub fn show_message(&self, msg:&str) {
		dialog::message(0, 0, msg);
	}//end show_message(&self, msg)

	/// # show_neighborhood_window(&self, nhood)
	/// 
	/// shows a new window with a colorful display of the specified neighborhood.
	pub fn update_neighborhood_tab(&mut self,nhood:&GroupInstance) {
		// try and re-initialize neighborhood_flex
		// self.neighborhood_flex = FlexGrid::default();
		// clear previous nonsense
		if self.neighborhood_flex.outer_flex.children() > 0 {
			self.neighborhood_flex.clear_inner_flexes();
		}//end if we have previous stuff to take care of
		// create and fill a grid of buttons
		let mut button_grid = Grid::new(nhood.sub_grid.rows(), nhood.sub_grid.cols());
		let button_width = get_default_grid_width() / button_grid.cols() as i32;
		let button_height = get_default_grid_height() / button_grid.rows() as i32;
		// don't show text if button is too small
		let show_label = button_width > get_max_grid_button_width() && button_height > get_max_grid_button_height();
		// print button size for debugging purposes
		println!("neighborhood button sizes w:{}\th:{}\tshow label:{}", button_width, button_height, show_label);
		// loop through and build the grid
		for row_idx in 0..button_grid.rows() {
			for col_idx in 0..button_grid.cols() {
				let this_building = nhood.sub_grid.get(row_idx, col_idx).unwrap();
				let mut this_button = Button::default()
					.with_size(button_width, button_height);
				// only add label if button big enough
				if show_label {
					this_button.set_label(format!("{}",this_building.build_type).as_str())
				}//end if we have room to show the label
				// start some color calculations
				let c = (this_building.rgb_color.0, this_building.rgb_color.1, this_building.rgb_color.2);
				// set color based on building
				this_button.set_color(Color::from_rgb(c.0, c.1, c.2));
				// do some calculations to determine if we should change label color
				let luminance = 0.299*c.0 as f32 + 0.587*c.1 as f32 + 0.114*c.2 as f32;
				if luminance > get_max_luminance_for_white_label() {
					this_button.set_label_color(Color::Black);
				}//end if label color should be black
				else { this_button.set_label_color(Color::White); }
				// TODO: set label color to negative of button color
				// update spot in button grid using through reference
				let this_spot = button_grid.get_mut(row_idx, col_idx).unwrap();
				*this_spot = this_button;
			}//end looping through columns of grid
		}//end looping through rows of grid
		// initialize our flex grid's size
		self.neighborhood_flex.initialize_flex(nhood.sub_grid.rows(), nhood.sub_grid.cols());
		// fill the flex_grid
		self.neighborhood_flex.fill_flex(&button_grid);
		// throw our buttons in the flex
		self.neighborhood_flex.buttons = button_grid;
		// actually try and make things show up
		if self.neighborhood_tab.children() < 1 {
			self.neighborhood_tab.add(&self.neighborhood_flex.outer_flex);
		}//end to tab if not there already
		self.neighborhood_flex.outer_flex.recalc();
		self.neighborhood_flex.outer_flex.redraw();
	}//end show_neighborhood_window(&self, nhood)

	/// # choose_district(&self)
	/// 
	/// opens dialog box prompting user to choose a district from the internal list
	/// 
	/// ## Return
	/// returns the index of self.districts that was selected.
	/// If the user cancelled the dialogue, then None will be returned.
	pub fn choose_district(&self) -> Option<usize> {
		let mut choose_district_dialog = "Enter the name of a district in the following list, case sensitive.".to_string();
		for district in &self.districts {
			choose_district_dialog = format!("{}\n{}", choose_district_dialog,district.name);
		}//end adding all the district names
		let choose_district_default:&str;
		if self.districts.len() > 0 {
			choose_district_default = &self.districts.first().unwrap().name;
		}//end if there is at least one option
		else {return None;}

		// loop to get dialog from the user
		loop {
			let result = dialog::input(0, 0, &choose_district_dialog, choose_district_default);

			if result.is_some() {
				let temp_result = result.clone().unwrap();
				for i in 0..self.districts.len() {
					if self.districts.get(i).unwrap().name.eq_ignore_ascii_case(&temp_result) {
						return Some(i);
					}//end if we found a match
				}//end checking each district for a match
			}//end if we got something to validate
			else {return None;}
		}//end looping until we get a result
	}//end choose_district(self)
}//end impl for gui

/// # FlexGrid
/// 
/// This struct is meant to be a sort of wrapper around a bunch of buttons and nested flexes in order to mimic a grid of buttons.
pub struct FlexGrid {
	/// # buttons
	/// The 2d array of buttons filling the grid
	pub buttons: Grid<Button>,
	/// # outer_flex
	/// The flex containing the flex containing the buttons
	pub outer_flex: Flex,
	/// # inner_flexes
	/// the flexes contained within the inner flex
	pub inner_flexes: Vec<Flex>,
}//end struct FlexGrid

impl FlexGrid {
	/// # default()
	/// 
	/// constructs the empty FlexGrid
	pub fn default() -> FlexGrid {
		FlexGrid {
			buttons:Grid::new(0,0),
			outer_flex:Flex::new(0, get_default_menu_height() + get_default_tab_padding(), get_default_grid_width(), get_default_grid_height(), None),
			inner_flexes:Vec::new(),
		}//end struct construction
	}//end new()

	/// # clear_inner_flexes
	/// 
	/// clears the children of this struct. should hopefully work
	pub fn clear_inner_flexes(&mut self) {
		self.outer_flex.clear();
		self.inner_flexes.clear();
		self.buttons.clear();
	}//end clear_inner_flexes(&mut self)

	/// #initialize_flex(self, grid)]
	/// 
	/// Sets up the flex-boxes like a grid
	pub fn initialize_flex(&mut self, rows:usize, cols:usize) {
		// set outer flex to be have rows of elements
		self.outer_flex.set_type(group::FlexType::Column);
		self.outer_flex.set_align(Align::LeftTop);
		for _row_index in 0..rows {
			let inner_flex_x = 0;//self.outer_flex.x();
			let inner_flex_y = self.outer_flex.y() + (self.outer_flex.width() / cols as i32);
			let inner_flex_w = get_default_grid_width() / cols as i32;
			let inner_flex_h = get_default_grid_height() / rows as i32;
			let mut inner_flex = Flex::new(inner_flex_x,inner_flex_y,inner_flex_w,inner_flex_h,None);
			inner_flex.set_type(group::FlexType::Row);
			// make flex show up
			self.outer_flex.add(&inner_flex);
			// save flex to struct
			self.inner_flexes.push(inner_flex);
		}//end adding inner flexes
		// println!("{} inner flexes", self.inner_flexes.len());
		// println!("inner flex x:{}", self.inner_flexes.first().unwrap().x());
	}//end initialize_flex(self, grid)

	/// # fill_flex(self, buttons)
	/// fills up the flex with buttons such that the buttons will show up in the flex looking like a grid
	/// 
	/// It should be noted that this function should expect to receive things in the order of col, rows
	pub fn fill_flex(&mut self, buttons:&Grid<Button>) {
		for row_idx in 0..buttons.rows() {
			let this_inner_flex = self.inner_flexes.get_mut(row_idx).unwrap();
			// loop over the current row of buttons
			for button in buttons.iter_row(row_idx) {
				if !button.was_deleted() {
					this_inner_flex.add(button);
				}//end if button wasn't deleted
				else {println!("button was deleted, row {}", row_idx);}
			}//end adding each button in row to inner flex
		}//end looping over each inner flex and adding buttons
	}//end fill_flex
}//end impl for FlexGrid

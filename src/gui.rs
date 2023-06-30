use fltk::app;
use fltk::app::App;
use fltk::app::Receiver;
use fltk::app::Sender;
use fltk::button::Button;
use fltk::dialog;
use fltk::enums::Align;
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
use grid::Grid;

use crate::grouping::Grouping;

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
	/// send messages for events
	pub msg_sender:Sender<MenuChoice>,
	// receive messages for events
	pub msg_receiver:Receiver<MenuChoice>,
	/// the menu bar at the top
	pub top_menu:SysMenuBar,
	/// the struct handling the 2d array of buttons for district representation
	pub grid_buttons:Vec<Vec<Button>>,
	/// the struct handling the grid of buttons for districts
	pub grid_flex: FlexGrid,
	/// group holding the various tabs
	pub tabs:Tabs,
	/// group holding the settings for generation
	pub settings_tab:Group,
	/// group holding the display of generated districts
	pub districts_tab:Group,
	/// the list of groupings that we'll generate districts from, each grouping is a district
	pub districts:Vec<Grouping>,
	/// The text buffer for displaying our list of the districts
	pub districts_list_buffer:TextBuffer,
}//end struct gui

fn get_default_win_width() -> i32 {900}
fn get_default_win_height() -> i32 {480}
fn get_default_menu_height() -> i32 {20}
fn get_default_tab_padding() -> i32 {20}
fn get_default_grid_width() -> i32 {get_default_win_width()}
fn get_default_grid_height() -> i32 {get_default_win_height()-get_default_menu_height() - get_default_tab_padding()}

impl GUI<'_> {
	/// # default()
	/// 
	/// 
	pub fn default<'a>(application:&'a App) -> GUI<'a> {
		let (s, r) = app::channel();
		let mut gui = GUI {
			application,
			main_window: Window::default(),
			msg_sender: s,
			msg_receiver: r,
			top_menu: SysMenuBar::default(),
			grid_buttons: Vec::new(),
			grid_flex: FlexGrid::default(),
			tabs: Tabs::default(),
			settings_tab: Group::default(),
			districts_tab: Group::default(),
			districts: Vec::new(),
			districts_list_buffer: TextBuffer::default(),
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

		// set default groupings
		self.districts.push(Grouping::new("slum".to_string()));
		self.districts.push(Grouping::new("suburb".to_string()));
		self.districts.push(Grouping::new("adventuring".to_string()));
		self.districts.push(Grouping::new("financial".to_string()));
		self.districts.push(Grouping::new("business".to_string()));

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
	}//end set_default_properties
	/// # initialize_top_menu
	/// 
	/// 
	pub fn initialize_top_menu(&mut self) {
		// set up all the emitters
		self.top_menu.add_emit(
			"&File/Choice1...\t",
			Shortcut::Ctrl | 'n',
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			MenuChoice::Choice1,
		);
		self.top_menu.add_emit(
			"&File/Choice2...\t",
			Shortcut::Ctrl | 'o',
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			MenuChoice::Choice2,
		);
		self.top_menu.add_emit(
			"Resize",
			Shortcut::Ctrl | 's',
			menu::MenuFlag::Normal,
			self.msg_sender.clone(),
			MenuChoice::Resize,
		);
	}//end initialize_top_menu
	/// # initialize grid
	/// 
	/// 
	pub fn initialize_grid(&mut self, ext_grid:Grid<String>) {
		// create a 2d array of buttons
		let mut button_grid: Vec<Vec<Button>> = Vec::new();
		// set size of buttons
		let button_width = get_default_grid_width() / ext_grid.cols() as i32;
		let button_height = get_default_grid_height() / ext_grid.rows() as i32;
		for row_index in 0..ext_grid.rows() as i32 {
			let mut temp_vec: Vec<Button> = Vec::new();
			for col_index in 0..ext_grid.cols() as i32 {
				let mut shrunk = ext_grid.get(row_index as usize, col_index as usize).unwrap().to_owned();
				if shrunk.len() > 9 {
					shrunk = shrunk[0..9].to_string();
				}//end if we need to shrink the name
				// make the button, positioned correctly
				let new_button = Button::default()
					.with_size(button_width, button_height)
					.with_label(&shrunk);
				// add the button to the list
				temp_vec.push(new_button);
			}//end converting each string into a button
			button_grid.push(temp_vec);
		}//end going through each row
		// save our hard-earned button grid in our struct
		self.grid_buttons = button_grid;
		// make the flex grid
		self.grid_flex.initialize_flex(ext_grid);
		self.grid_flex.fill_flex(&self.grid_buttons);
		// reposition flex grid because it likes to get lost (also screws up resizing for some ungodly reason)
		// self.grid_flex.outer_flex.resize(0 - button_width / 2, self.districts_tab.y(), self.grid_flex.outer_flex.width() + button_width / 2, self.grid_flex.outer_flex.height());
		// actually make the grid show up
		self.districts_tab.add_resizable(&self.grid_flex.outer_flex);
	}//end initialize_grid
	/// # initialize_setting(self)
	/// 
	/// 
	pub fn initialize_settings(&mut self) {
		// int inputs for grid rows and columns
		let mut grid_rows_input = IntInput::default()
			.with_size(50, 20)
			.with_pos(100, 50)
			.with_label("Grid Rows");
		grid_rows_input.set_value("10");
		let mut grid_cols_input = IntInput::default()
			.with_size(50, 20)
			.right_of(&grid_rows_input, 120)
			.with_label("Grid Columns");
		grid_cols_input.set_value("10");

		// buttons for editing districts
		let mut set_color_button = Button::default()
			.with_size(130, 30)
			.with_pos(50, 100)
			.with_label("Set Color...");
		set_color_button.emit(self.msg_sender.clone(), MenuChoice::SetColor);
		let mut add_district_button = Button::default()
			.with_size(130, 30)
			.below_of(&set_color_button, 10)
			.with_label("Add District...");
		add_district_button.emit(self.msg_sender.clone(), MenuChoice::AddDistrict);
		let mut remove_district_button = Button::default()
			.with_size(130, 30)
			.below_of(&add_district_button, 10)
			.with_label("Remove District...");
		remove_district_button.emit(self.msg_sender.clone(), MenuChoice::RemoveDistrict);

		// button for generating districts
		let mut gen_districts_button = Button::default()
			.with_size(150, 40)
			.below_of(&remove_district_button, 50)
			.with_label("Generate Districts");
		gen_districts_button.emit(self.msg_sender.clone(), MenuChoice::GenerateDistricts);

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
		self.settings_tab.add(&grid_rows_input);
		self.settings_tab.add(&grid_cols_input);
		self.settings_tab.add(&set_color_button);
		self.settings_tab.add(&add_district_button);
		self.settings_tab.add(&remove_district_button);
		self.settings_tab.add(&gen_districts_button);
		self.settings_tab.add(&dist_list_disp);
	}//end initialize_settings(self)

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

	/// # show(self)
	/// 
	/// Simply causes the gui to become visible, or returns an error if it can't
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
	pub buttons: Vec<Vec<Button>>,
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
			buttons:Vec::new(),
			outer_flex:Flex::new(0, get_default_menu_height() + get_default_tab_padding(), get_default_grid_width(), get_default_grid_height(), None),
			inner_flexes:Vec::new(),
		}//end struct construction
	}//end new()

	/// #initialize_flex(self, grid)]
	/// 
	/// Sets up the flex-boxes like a grid
	pub fn initialize_flex(&mut self, grid:Grid<String>) {
		// set outer flex to be have rows of elements
		self.outer_flex.set_type(group::FlexType::Row);
		self.outer_flex.set_align(Align::LeftTop);
		for _row_index in 0..grid.rows() {
			let inner_flex_x = 0;//self.outer_flex.x();
			let inner_flex_y = self.outer_flex.y() + (self.outer_flex.width() / grid.cols() as i32);
			let inner_flex_w = get_default_grid_width() / grid.cols() as i32;
			let inner_flex_h = get_default_grid_height() / grid.rows() as i32;
			let mut inner_flex = Flex::new(inner_flex_x,inner_flex_y,inner_flex_w,inner_flex_h,None);
			inner_flex.set_type(group::FlexType::Column);
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
	pub fn fill_flex(&mut self, buttons:&Vec<Vec<Button>>) {
		for row_index in 0..buttons.len() {
			let this_inner_flex = self.inner_flexes.get_mut(row_index).unwrap();
			let this_button_row = buttons.get(row_index).unwrap();
			for button in this_button_row {
				this_inner_flex.add(button);
			}//end adding each button in row to inner flex
		}//end looping over each inner flex and adding buttons
	}//end fill_flex
}//end impl for FlexGrid

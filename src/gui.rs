use fltk::app;
use fltk::app::App;
use fltk::app::Receiver;
use fltk::app::Sender;
use fltk::button::Button;
use fltk::enums::FrameType;
use fltk::enums::Shortcut;
use fltk::group;
use fltk::group::Flex;
use fltk::menu;
use fltk::menu::SysMenuBar;
use fltk::prelude::FltkError;
use fltk::prelude::GroupExt;
use fltk::prelude::MenuExt;
use fltk::prelude::WidgetBase;
use fltk::prelude::WidgetExt;
use fltk::window::Window;
use grid::Grid;

#[derive(Clone)]
pub enum MenuChoice {
	Choice1,
	Choice2,
	Resize
}//end enum MenuChoice

pub struct GUI {
	pub application:App,
	pub main_window:Window,
	pub menu_send_receive:(Sender<MenuChoice>,Receiver<MenuChoice>),
	pub top_menu:SysMenuBar,
	pub grid_buttons:Vec<Vec<Button>>,
	pub grid_flex: FlexGrid,
}//end struct gui

fn get_default_win_width() -> i32 {900}
fn get_default_win_height() -> i32 {480}
fn get_default_menu_height() -> i32 {20}
fn get_default_grid_width() -> i32 {get_default_win_width()}
fn get_default_grid_height() -> i32 {get_default_win_height()-get_default_menu_height()}

impl GUI {
	/// # default()
	/// 
	/// 
	pub fn default() -> GUI {
		let mut gui = GUI {
			application: App::default(),
			main_window: Window::default(),
			menu_send_receive: app::channel(),
			top_menu:SysMenuBar::default(),
			grid_buttons:Vec::new(),
			grid_flex:FlexGrid::default(),
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

		// top menu settings
		self.top_menu = self.top_menu.clone()
			.with_size(get_default_win_width(), get_default_menu_height());
		self.top_menu.set_frame(FrameType::FlatBox);
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
			self.menu_send_receive.0.clone(),
			MenuChoice::Choice1,
		);
		self.top_menu.add_emit(
			"&File/Choice2...\t",
			Shortcut::Ctrl | 'o',
			menu::MenuFlag::Normal,
			self.menu_send_receive.0.clone(),
			MenuChoice::Choice2,
		);
		self.top_menu.add_emit(
			"Resize",
			Shortcut::Ctrl | 's',
			menu::MenuFlag::Normal,
			self.menu_send_receive.0.clone(),
			MenuChoice::Resize,
		);
		// actually make the menu show up
		self.main_window.add(&self.top_menu);
	}//end initialize_top_menu
	/// # initialize grid
	/// 
	/// 
	pub fn initialize_grid(&mut self, ext_grid:Grid<String>) {
		// create a 2d array of buttons
		let mut button_grid: Vec<Vec<Button>> = Vec::new();
		for row_index in 0..ext_grid.rows() as i32 {
			let mut temp_vec: Vec<Button> = Vec::new();
			for col_index in 0..ext_grid.cols() as i32 {
				// make the button, positioned correctly
				let button_width = get_default_grid_width() / ext_grid.cols() as i32;
				let button_height = get_default_grid_height() / ext_grid.rows() as i32;
				let new_button = Button::default()
					.with_size(button_width, button_height)
					.with_label(ext_grid.get(row_index as usize, col_index as usize).unwrap());
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
		// reposition flex grid because it likes to get lost
		self.grid_flex.outer_flex.set_pos(0, self.top_menu.height());
		// actually make the grid show up
		self.main_window.add(&self.grid_flex.outer_flex);
	}//end initialize_grid
	/// # show(self)
	/// 
	/// Simply causes the gui to become visible, or returns an error if it can't
	pub fn show(&mut self) -> Result<(), FltkError> {
		self.grid_flex.outer_flex.recalc();
		self.main_window.show();
		self.application.run()
	}//end show(&mut self)
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
			outer_flex:Flex::new(0, get_default_menu_height(), get_default_grid_width(), get_default_grid_height(), None),
			inner_flexes:Vec::new(),
		}//end struct construction
	}//end new()

	/// #initialize_flex(self, grid)]
	/// 
	/// Sets up the flex-boxes like a grid
	pub fn initialize_flex(&mut self, grid:Grid<String>) {
		// set outer flex to be have rows of elements
		self.outer_flex.set_type(group::FlexType::Row);
		for _row_index in 0..grid.rows() {
			let inner_flex_x = self.outer_flex.x();
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

use fltk::app;
use fltk::app::App;
use fltk::app::Receiver;
use fltk::app::Sender;
use fltk::button;
use fltk::button::Button;
use fltk::enums::FrameType;
use fltk::enums::Shortcut;
use fltk::group;
use fltk::group::Tile;
use fltk::menu;
use fltk::menu::SysMenuBar;
use fltk::prelude::FltkError;
use fltk::prelude::GroupExt;
use fltk::prelude::MenuExt;
use fltk::prelude::TableExt;
use fltk::prelude::WidgetExt;
use fltk::table::Table;
use fltk::window;
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
	pub grid_container:Tile,
	pub grid_buttons:Vec<Vec<Button>>,
}//end struct gui

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
			grid_container:Tile::default(),
			grid_buttons:Vec::new(),
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

		// grid settings
		self.grid_container = self.grid_container.clone()
			.with_size(get_default_grid_width(), get_default_grid_height())
			.with_pos(0, get_default_menu_height())
			.below_of(&self.top_menu, 0);
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
					.with_pos(self.grid_container.x() + col_index * button_width, self.grid_container.y() + row_index * button_height)
					.with_label(ext_grid.get(row_index as usize, col_index as usize).unwrap());
				// add the button to the list
				temp_vec.push(new_button);
			}//end converting each string into a button
			button_grid.push(temp_vec);
		}//end going through each row
		// save our hard-earned button grid in our struct
		self.grid_buttons = button_grid;
		// add all those buttons to our grid
		for row_index in 0..self.grid_buttons.len() {
			for col_index in 0..self.grid_buttons.get(0).unwrap().len() {
				self.grid_container.add_resizable(self.grid_buttons.get(row_index).unwrap().get(col_index).unwrap());
			}//end looping over inner vector of buttons
		}//end looping over outer vector of buttons
		// actually make the grid show up
		self.main_window.add(&self.grid_container);
	}//end initialize_grid
	/// # show(self)
	/// 
	/// Simply causes the gui to become visible, or returns an error if it can't
	pub fn show(&mut self) -> Result<(), FltkError> {
		self.main_window.show();
		self.application.run()
	}//end show(&mut self)
}//end impl for gui

fn get_default_win_width() -> i32 {900}
fn get_default_win_height() -> i32 {480}
fn get_default_menu_height() -> i32 {20}
fn get_default_grid_width() -> i32 {get_default_win_width()}
fn get_default_grid_height() -> i32 {get_default_win_height()-get_default_menu_height()}
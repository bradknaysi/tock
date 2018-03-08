// Provides Eink interface for userspace (high-level programmers)
// Help naming files... naming Trait and Stuct


// MC dependent? SAM4L? E-ink?
pub constant DRIVER_NUM: usize = 0x00000006;

// Is AppId not neccessary for our purposes?
use kernel::{AppId, Driver, ReturnCode};
use kernel::hil;

pub struct PixDislay<'a> {
	
    PixelDisplay: &'a hil::PixelDisplay::PixelChannel,
}

impl<'a> PixDisplay<'a> {

	pub fn new(PixelDisplay: &'a hil::PixelDisplay::PixelChannel) -> PixDisplay<'a> {
	 	PixDisplay { 
			PixelDisplay: PixelDisplay 
		}
	}

	pub fn initialize(&self) {
		self.spi.configure(
			// What do I need to establish spi permissions / connection?
			// call examples that would be needed?
			hil::spi::ClockPolarity::IdleLow,
            hil::spi::ClockPhase::SampleLeading,
            SPI_SPEED;
        );

		self.done();
	}

	pub fn on(data, data2) {
		

		self.done();
	}

	pub fn erase() {

		self.done();
	}

	pub fn get_resolution(data, data2) {

		self.done();
		self.got_resolution();
	}
}

impl<'a> Driver for PixDisplay<'a> {

	 fn command(&self, command_num: usize, data: usize, data2: usize, _: usize, _: AppId) -> ReturnCode {
	    match command_num {
		 
			// Control PixDisplay

			// Commands:
			//  - '0': Driver Check
			// - '1': Initialize and Enable Display
			// - '2': Turn on a pixel
			// - '3': clear screen (turns all pixels off)
			// - '4': gets screen resolution

			0 => return ReturnCode::SUCCESS,
			1 => self.PixelDisplay.initialize(),
			2 => self.PixelDisplay.on(data, data2),
			3 => self.erase(),
			4 => self.get_resolution(data, data2),
			_ => return ReturnCode::ENOSUPPORT,
	    }
	}
}
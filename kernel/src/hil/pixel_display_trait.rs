// Interface for PixelDisplay trait... allowing pixel_display layer to communicate with Eink driver layer

use returncode::ReturnCode;

// Interface for using Eink

pub trait PixelChannel {

    // deals with initial setup?
    fn initialize(&self) -> ReturnCode;

    // sets a given (x,y) pixel
    fn on(x: i32, y: i32) -> ReturnCode; 

    // sets all pixels on screen to 0?
    fn erase(&self) -> ReturnCode;

    // gets screen resolution of device
    fn get_resolution(x: i32, y: i32) -> ReturnCode;
    
}

pub trait Client {

    // Called when pixel is being set (on() or erase() function)
    fn done(&self);

    // Called when trying to get the screen's resolution (get_resolution function)
    fn got_resolution(&self);
}
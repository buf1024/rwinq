use rwqstrategy::Strategy;
use test_strategy::TestStrategy;

mod test_strategy;

#[no_mangle]
pub extern "C" fn new_strategy() -> *mut libc::c_void {
    let data: Box<Box<dyn Strategy>> = Box::new(Box::new(TestStrategy {}));
    Box::into_raw(data) as *mut Box<dyn Strategy> as *mut libc::c_void
}

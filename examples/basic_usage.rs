/// Shows how to use the lower-level evdi-sys with the help of some helpers
/// from the higher level library.
use std::ffi::c_void;
use std::os::raw::{c_int, c_uint};
use std::ptr::NonNull;
use std::time::{Duration, Instant};

use evdi_sys::*;

use std::{io, thread};

const DEVICE: u32 = 1;
const RUN_FOR: Duration = Duration::from_secs(30);

extern "C" fn mode_changed_handler(mode: evdi_mode, _: *mut c_void) {
    eprintln!("Mode: {:?}", mode);
}

extern "C" fn cursor_move_handler(cursor_move: evdi_cursor_move, _: *mut c_void) {
    eprintln!("Move {:?}", cursor_move);
}

extern "C" fn cursor_set_handler(cursor_set: evdi_cursor_set, _: *mut c_void) {
    eprintln!("Set {:?}", cursor_set);
}

extern "C" fn update_ready_handler(_buffer_id: c_int, handle: *mut c_void) {
    let mut rects = vec![
        evdi_rect {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        };
        16
    ]
        .into_boxed_slice();
    unsafe {
        evdi_grab_pixels(handle as evdi_handle, rects.as_mut_ptr(), &mut 0);
    }
}

fn main() {
    unsafe {
        let handle = NonNull::new(evdi_open(DEVICE as c_int)).unwrap();

        let width: i32 = 1280;
        let height: i32 = 800;
        let edid = include_bytes!("sample_edid_1280_800");
        evdi_connect(
            handle.as_ptr(),
            edid.as_ptr(),
            edid.len() as c_uint,
            (width * height) as u32,
        );

        let fd = evdi_get_event_ready(handle.as_ptr());

        let bits_per_pixel = 4; // Assumed for simplicity. This may be invalid depending on the evdi_mode of your display
        let mut buffer = vec![0u8; (width * height) as usize].into_boxed_slice();
        let mut rects = vec![
            evdi_rect {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0
            };
            16
        ]
            .into_boxed_slice();

        evdi_register_buffer(
            handle.as_ptr(),
            evdi_buffer {
                id: 1,
                buffer: buffer.as_mut_ptr() as *mut c_void,
                width,
                height,
                stride: bits_per_pixel / 8 * width,
                rects: rects.as_mut_ptr(),
                rect_count: 0,
            },
        );

        let poller = Poller::new(fd);

        if !poller.poll_ready(Some(Duration::from_secs(5))) {
            panic!("Timeout waiting for initial ready");
        }

        evdi_enable_cursor_events(handle.as_ptr(), true);

        spawn_event_dispatcher(handle, poller);

        let start = Instant::now();
        loop {
            if Instant::now() - start > RUN_FOR {
                break;
            }

            if evdi_request_update(handle.as_ptr(), 1) {
                evdi_grab_pixels(handle.as_ptr(), rects.as_mut_ptr(), &mut 0);
            }

            thread::sleep(Duration::from_millis(1000 / 60));
        }

        evdi_disconnect(handle.as_ptr());
        evdi_close(handle.as_ptr());
    }
}

fn spawn_event_dispatcher(handle: NonNull<evdi_device_context>, poller: Poller) {
    struct Wrapper(NonNull<evdi_device_context>);
    unsafe impl Send for Wrapper {}
    unsafe impl Sync for Wrapper {}
    let wrapper = Wrapper(handle);

    thread::spawn(move || {
        let handle = wrapper.0;
        let mut ctx = evdi_event_context {
            dpms_handler: None,
            mode_changed_handler: Some(mode_changed_handler),
            update_ready_handler: Some(update_ready_handler),
            crtc_state_handler: None,
            cursor_set_handler: Some(cursor_set_handler),
            cursor_move_handler: Some(cursor_move_handler),
            ddcci_data_handler: None,
            // Safety: We cast to a mut pointer, but we never cast back to a mut reference
            user_data: handle.as_ptr() as *mut c_void,
        };

        loop {
            if !poller.poll_ready(None) {
                eprintln!("Failed to poll ready");
                return;
            }

            unsafe {
                evdi_handle_events(handle.as_ptr(), &mut ctx);
            };
        }
    });
}

struct Poller(i32);

impl Poller {
    fn new(fd: i32) -> Self {
        Self(fd)
    }

    fn poll_ready(&self, timeout: Option<Duration>) -> bool {
        let mut pollfd = libc::pollfd {
            fd: self.0,
            events: libc::POLLIN,
            revents: 0,
        };

        let timeout = timeout.map(|t| t.as_millis() as i32).unwrap_or(-1);

        unsafe {
            let ret = libc::poll(&mut pollfd, 1, timeout);
            if ret < 0 {
                let err = io::Error::last_os_error();
                panic!("Error polling: {:?}", err);
            }
        }

        pollfd.revents == libc::POLLIN
    }
}

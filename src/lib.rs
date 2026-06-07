extern crate winapi;

pub mod event;
mod keyboard;
mod mouse;

pub use event::*;
use keyboard::*;
use mouse::*;
use std::collections::{HashMap, VecDeque};
use std::ffi::OsStr;
use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use std::ptr;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::thread::JoinHandle;
use winapi::shared::minwindef;
use winapi::shared::windef;
use winapi::um::libloaderapi;
use winapi::um::winnt;
use winapi::um::winuser;

#[repr(C)]
struct RAWINPUTHID {
    pub header: winuser::RAWINPUTHEADER,
    pub data: winuser::RAWHID,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RAWINPUTMOUSE {
    pub header: winuser::RAWINPUTHEADER,
    pub data: winuser::RAWMOUSE,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RAWINPUTKEYBOARD {
    pub header: winuser::RAWINPUTHEADER,
    pub data: winuser::RAWKEYBOARD,
}

enum RAWINPUTTYPE {
    MOUSE(*mut RAWINPUTMOUSE),
    KEYBOARD(*mut RAWINPUTKEYBOARD),
    HID(*mut RAWINPUTHID),
}

unsafe fn derive_rawinput_type(input: *mut winuser::RAWINPUT) -> RAWINPUTTYPE {
    unsafe {
        match (*input).header.dwType {
            winuser::RIM_TYPEMOUSE => RAWINPUTTYPE::MOUSE(input as *mut RAWINPUTMOUSE),
            winuser::RIM_TYPEKEYBOARD => RAWINPUTTYPE::KEYBOARD(input as *mut RAWINPUTKEYBOARD),
            winuser::RIM_TYPEHID => RAWINPUTTYPE::HID(input as *mut RAWINPUTHID),
            _ => panic!("Should be Unreachable!"),
        }
    }
}

/// Mouse Raw Input Name
#[derive(Clone)]
pub struct Mouse {
    name: String,
}

/// Keyboard Raw Input Name
#[derive(Clone)]
pub struct Keyboard {
    name: String,
}

/// Hid Raw Input Name
#[derive(Clone)]
pub struct Hid {
    name: String,
}

/// Stores Names to All Raw Input Devices
#[derive(Clone)]
pub struct Devices {
    pub mice: Vec<Mouse>,
    pub keyboards: Vec<Keyboard>,
    pub hids: Vec<Hid>,
    device_map: HashMap<winnt::HANDLE, usize>,
}

impl Devices {
    pub fn new() -> Devices {
        Devices {
            mice: Vec::new(),
            keyboards: Vec::new(),
            hids: Vec::new(),
            device_map: HashMap::new(),
        }
    }
}

enum Command {
    Register(DeviceType),
    GetEvent,
    Finish,
}

/// Types of Raw Input Device
#[derive(PartialEq, Eq)]
pub enum DeviceType {
    Mice,
    Keyboards,
    Hids,
    All,
}

/// Manages Raw Input Processing
pub struct RawInputManager {
    joiner: Option<JoinHandle<()>>,
    sender: Sender<Command>,
    receiver: Receiver<Option<RawEvent>>,
}

impl RawInputManager {
    pub fn new() -> Result<RawInputManager, &'static str> {
        let (tx, rx) = channel();
        let (tx2, rx2) = channel();

        let joiner = thread::spawn(move || {
            let hwnd = setup_message_window();
            let mut event_queue = VecDeque::new();
            let mut devices = Devices::new();

            let mut exit = false;
            while !exit {
                match rx.recv().unwrap() {
                    Command::Register(thing) => {
                        devices = register_devices(hwnd, thing).unwrap();
                        tx2.send(None).unwrap();
                    }
                    Command::GetEvent => {
                        tx2.send(get_event(&mut event_queue, &devices)).unwrap();
                    }
                    Command::Finish => {
                        exit = true;
                    }
                };
            }
        });
        Ok(RawInputManager {
            joiner: Some(joiner),
            sender: tx,
            receiver: rx2,
        })
    }

    /// Allows Raw Input devices of type device_type to be received from the Input Manager
    pub fn register_devices(&mut self, device_type: DeviceType) {
        self.sender.send(Command::Register(device_type)).unwrap();
        self.receiver.recv().unwrap();
    }

    /// Get Event from the Input Manager
    pub fn get_event(&mut self) -> Option<RawEvent> {
        self.sender.send(Command::GetEvent).unwrap();
        self.receiver.recv().unwrap()
    }
}

impl Drop for RawInputManager {
    fn drop(&mut self) {
        self.sender.send(Command::Finish).unwrap();
        self.joiner.take().unwrap().join().unwrap();
    }
}

fn register_devices(hwnd: windef::HWND, reg_type: DeviceType) -> Result<Devices, &'static str> {
    let mut rid_vec: Vec<winuser::RAWINPUTDEVICE> = Vec::new();
    if (reg_type == DeviceType::Mice) || (reg_type == DeviceType::All) {
        rid_vec.push(winuser::RAWINPUTDEVICE {
            usUsagePage: 1,
            usUsage: 2, // Mice
            dwFlags: winuser::RIDEV_INPUTSINK,
            hwndTarget: hwnd,
        });
    }
    if (reg_type == DeviceType::Hids) || (reg_type == DeviceType::All) {
        rid_vec.push(winuser::RAWINPUTDEVICE {
            usUsagePage: 1,
            usUsage: 5, // Gamepads
            dwFlags: winuser::RIDEV_INPUTSINK,
            hwndTarget: hwnd,
        });
    }
    if (reg_type == DeviceType::Keyboards) || (reg_type == DeviceType::All) {
        rid_vec.push(winuser::RAWINPUTDEVICE {
            usUsagePage: 1,
            usUsage: 6, // Keyboards
            dwFlags: winuser::RIDEV_INPUTSINK,
            hwndTarget: hwnd,
        });
    }

    unsafe {
        if winuser::RegisterRawInputDevices(
            rid_vec.as_mut_ptr(),
            rid_vec.len() as u32,
            mem::size_of::<winuser::RAWINPUTDEVICE>() as u32,
        ) == 0
        {
            return Err("Registration of Controller Failed");
        }
    }
    Ok(produce_raw_device_list())
}

fn read_input_buffer(event_queue: &mut VecDeque<RawEvent>, devices: &Devices) {
    let mut rawinput_alloc = mem::MaybeUninit::<winuser::RAWINPUT>::uninit();
    let mut buffer_size: u32 = 0;

    let mut numberofelements: i32 = unsafe {
        winuser::GetRawInputBuffer(
            ptr::null_mut(),
            &mut buffer_size,
            mem::size_of::<winuser::RAWINPUTHEADER>() as u32,
        ) as i32
    };
    if numberofelements as i32 == -1 {
        panic!("GetRawInputBuffer Gave Error on First Call!");
    }
    buffer_size = 1024;
    numberofelements = unsafe {
        winuser::GetRawInputBuffer(
            rawinput_alloc.as_mut_ptr(),
            &mut buffer_size,
            mem::size_of::<winuser::RAWINPUTHEADER>() as u32,
        ) as i32
    };
    if numberofelements as i32 == -1 {
        panic!("GetRawInputBuffer Gave Error on Second Call!");
    }

    let rawinput = unsafe { rawinput_alloc.assume_init() };
    for _ in 0..numberofelements as u32 {
        let raw_input_ptr = unsafe { derive_rawinput_type(rawinput_alloc.as_mut_ptr()) };
        let pos = match devices.device_map.get(&rawinput.header.hDevice) {
            Some(item) => *item,
            None => continue,
        };
        match raw_input_ptr {
            RAWINPUTTYPE::MOUSE(pointer) => {
                let value = unsafe { *pointer };
                event_queue.extend(process_mouse_data(&value.data, pos));
            }
            RAWINPUTTYPE::KEYBOARD(pointer) => {
                let value = unsafe { *pointer };
                event_queue.extend(process_keyboard_data(&value.data, pos));
            }
            _ => (),
        }
    }
}

fn get_event(event_queue: &mut VecDeque<RawEvent>, devices: &Devices) -> Option<RawEvent> {
    if event_queue.is_empty() {
        read_input_buffer(event_queue, &devices);
    }
    let event = event_queue.pop_front();
    event
}

fn setup_message_window() -> windef::HWND {
    let hwnd: windef::HWND;
    unsafe {
        let hinstance = libloaderapi::GetModuleHandleW(ptr::null());
        if hinstance == ptr::null_mut() {
            panic!("Instance Generation Failed");
        }
        let classname = OsStr::new("RawInput Hidden Window")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        let wcex = winuser::WNDCLASSEXW {
            cbSize: (mem::size_of::<winuser::WNDCLASSEXW>()) as u32,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hbrBackground: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hIcon: ptr::null_mut(),
            hIconSm: ptr::null_mut(),
            hInstance: hinstance,
            lpfnWndProc: Some(winuser::DefWindowProcW),
            lpszClassName: classname.as_ptr(),
            lpszMenuName: ptr::null_mut(),
            style: 0,
        };
        let a = winuser::RegisterClassExW(&wcex);
        if a == 0 {
            panic!("Registering WindowClass Failed!");
        }

        hwnd = winuser::CreateWindowExW(
            0,
            classname.as_ptr(),
            classname.as_ptr(),
            0,
            winuser::CW_USEDEFAULT,
            winuser::CW_USEDEFAULT,
            winuser::CW_USEDEFAULT,
            winuser::CW_USEDEFAULT,
            winuser::HWND_MESSAGE,
            ptr::null_mut(),
            hinstance,
            ptr::null_mut(),
        );
        if hwnd.is_null() {
            panic!("Window Creation Failed!");
        }
    }
    hwnd
}

/// Produces a Device struct containing ID's to all available raw input Devices
pub fn produce_raw_device_list() -> Devices {
    let mut device_list = Devices::new();
    let mut buffer = mem::MaybeUninit::<[winuser::RAWINPUTDEVICELIST; 500]>::uninit();
    let mut num_devices: u32 = 0;
    let cb_size = mem::size_of::<winuser::RAWINPUTDEVICELIST>() as u32;
    let mut result =
        unsafe { winuser::GetRawInputDeviceList(ptr::null_mut(), &mut num_devices, cb_size) };
    if result == -1i32 as u32 {
        panic!("Failed to Get Raw Device List!");
    }
    result = unsafe {
        winuser::GetRawInputDeviceList(
            buffer.as_mut_ptr() as *mut winuser::RAWINPUTDEVICELIST,
            &mut num_devices,
            cb_size,
        )
    };
    if result == -1i32 as u32 {
        panic!("Failed to Get Raw Device List Again!");
    }
    let buffer = unsafe { buffer.assume_init() };

    for pos in 0..result as usize {
        let device = buffer[pos];
        let device_handle = device.hDevice;
        let device_type = device.dwType;
        let mut name_buffer = mem::MaybeUninit::<[u16; 1024]>::uninit();
        let mut name_buffer_size: u32 = 1024;
        let result_2 = unsafe {
            winuser::GetRawInputDeviceInfoW(
                device_handle,
                winuser::RIDI_DEVICENAME,
                name_buffer.as_mut_ptr() as minwindef::LPVOID,
                &mut name_buffer_size,
            )
        };
        if result_2 == -1i32 as u32 {
            panic!(
                "GetRawInputDeviceInfo Failed: Required Size: {:?}",
                name_buffer_size
            );
        }
        let name_slice = unsafe { &name_buffer.assume_init()[0..result_2 as usize] };
        let full_name = match OsString::from_wide(name_slice).into_string() {
            Ok(something) => something,
            Err(_) => panic!("String Conversion Failed"),
        };

        let name = String::from(full_name);

        match device_type {
            winuser::RIM_TYPEMOUSE => {
                if let Some(pos) = device_list
                    .mice
                    .iter()
                    .cloned()
                    .enumerate()
                    .find(|m| m.1.name == name)
                {
                    device_list.device_map.insert(device_handle, pos.0);
                } else {
                    device_list
                        .device_map
                        .insert(device_handle, device_list.mice.len());
                    device_list.mice.push(Mouse { name: name });
                }
            }
            winuser::RIM_TYPEKEYBOARD => {
                if let Some(pos) = device_list
                    .keyboards
                    .iter()
                    .cloned()
                    .enumerate()
                    .find(|m| m.1.name == name)
                {
                    device_list.device_map.insert(device_handle, pos.0);
                } else {
                    device_list
                        .device_map
                        .insert(device_handle, device_list.keyboards.len());
                    device_list.keyboards.push(Keyboard { name: name });
                }
            }
            winuser::RIM_TYPEHID => {
                if let Some(pos) = device_list
                    .hids
                    .iter()
                    .cloned()
                    .enumerate()
                    .find(|m| m.1.name == name)
                {
                    device_list.device_map.insert(device_handle, pos.0);
                } else {
                    device_list
                        .device_map
                        .insert(device_handle, device_list.hids.len());
                    device_list.hids.push(Hid { name: name });
                }
            }
            _ => (),
        }
    }
    device_list
}

/// Prints a list of all available raw input devices
pub fn print_raw_device_list() {
    let device_list = produce_raw_device_list();
    println!("Mice:");
    for mouse in device_list.mice {
        println!("{}", mouse.name);
    }
    println!("Keyboards:");
    for keyboard in device_list.keyboards {
        println!("{}", keyboard.name);
    }
    println!("Hids:");
    for hid in device_list.hids {
        println!("{}", hid.name);
    }
}

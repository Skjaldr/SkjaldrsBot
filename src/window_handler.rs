use std::ffi::CString;
use std::{ptr, thread,  time::Duration};
use windows_sys::{ Win32::Foundation::*, 
    //Win32::System::StationsAndDesktops, 
    Win32::UI::WindowsAndMessaging::*,
    //Win32::UI::WindowsAndMessaging,
};

const KEY_DOWN: u32 = 0x0100;
const KEY_UP: u32 = 0x0101;
const WM_CHAR: u32 = 0x0102;

// Find the specified window
pub unsafe fn get_window() -> HWND {
    
        let sb = CString::new("Shadowbane").expect("CString::new failed");
        let sbptr = sb.as_ptr();
        let hwnd = FindWindowA(ptr::null(), sbptr as *mut u8);
        hwnd
}

//for our logic to work in renameing the window and passing the renamed window into
//the commands in order for discord to execute to ONLY the renamed window without
//renaming other windows, we need to separate the step s into 3 different functions.
// We already have the get_window() function.  This takes in the window named "Shadowbane"
// turns it into a pointer, and stores the FindWindowA into hwnd.  Then returns that hwnd.

//What is needed next however, is the rename function.  We need to take in that window only, and rename it
pub unsafe fn rename_window() -> HWND {
	let sb_renamed = CString::new("Shadowbane_bot").expect("Failed to read CString");
	let renamed_ptr = sb_renamed.as_ptr();
    
	let orig_window = get_window();
	let renamed = SetWindowTextA(orig_window, renamed_ptr as *mut u8) as isize;
	renamed
}

pub unsafe fn get_renamed() -> HWND {
	let renamed: HWND = rename_window();
	renamed
    	
}

// This function takes in the window handle (HWND) and the players name (p_name)
// We then send a command to the Shadowbane client to open the summon spell
// where the function will then iterate over the player's name and type the name
// into the text box.  After the name is input, the function will execute the spell.

pub async fn summon_player(p_name: &str) {

    unsafe {
        
        let hwnd = get_window();

        SendMessageA(hwnd, KEY_DOWN, 'B' as usize, 0);
        //thread::sleep(Duration::from_nanos(10));
        
        SendMessageA(hwnd, KEY_UP, 'B' as usize, 0);
        
        for i in p_name.chars() {
            SendMessageA(hwnd, WM_CHAR, i as usize, 0);
        }
        
        thread::sleep(Duration::from_nanos(1000));
        SendMessageA(hwnd, KEY_DOWN, 0x0D, 0);
        println!("I'm calling the summon_player function!");   
    }
}

// This function takes in the window handle and the player name, then press
// L is the key bound to the /guildinv command in game, the function will then
// input the player name and invite them to the guild

pub async fn guild_inv(player_name: &str) {

    unsafe {
        let hwnd = get_window();
        println!("I am calling the guild invite function!");
        SendMessageA(hwnd, KEY_DOWN, 'L' as usize, 0);
        thread::sleep(Duration::from_nanos(100));
        SendMessageA(hwnd, KEY_UP, 'L' as usize, 0);

        for i in player_name.chars() {
            SendMessageA(hwnd, WM_CHAR, i as usize, 0);
        }

        thread::sleep(Duration::from_nanos(10000));
        SendMessageA(hwnd, KEY_DOWN, 0x0D, 0);
    }
}



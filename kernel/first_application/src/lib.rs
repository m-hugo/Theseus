//! This crate contains a simple routine to start the first application (or set of applications). 
//! 
//! This should be invoked at or towards the end of the kernel initialization procedure. 
//!
//! ## Important Dependency Note
//!  
//! In general, Theseus kernel crates *cannot* depend on application crates.
//! However, this crate is a special exception in that it directly loads and runs
//! the first application crate.
//! 
//! Thus, it's safest to ensure that first application crate is always included
//! in the build by specifying it as a direct dependency here.
//! 
//! Currently, that crate is `applications/shell`, but if it changes,
//! we should change that dependendency in this crates `Cargo.toml` manifest.

#![no_std]

extern crate alloc;
#[macro_use] extern crate log;
extern crate spawn;
extern crate mod_mgmt;
extern crate path;
extern crate window_manager;
extern crate mpmc;
extern crate event_types;
extern crate multicore_bringup;
extern crate framebuffer;
extern crate memory;
extern crate spin;


use spin::{Mutex, Once};
use alloc::sync::{Arc, Weak};
use memory::{EntryFlags, MappedPages, PhysicalAddress};
use framebuffer::{Framebuffer, AlphaPixel, Pixel};
use mpmc::Queue;
use event_types::{Event, MousePositionEvent};
use alloc::string::ToString;
use mod_mgmt::CrateNamespace;
use path::Path;

/// See the crate-level docs and this crate's `Cargo.toml` for more.
const FIRST_APPLICATION_CRATE_NAME: &'static str = "window_manager-";


/// Initializes the final framebuffer based on VESA graphics mode information obtained during boot.
///
/// The final framebuffer represents the actual pixel content displayed on screen
/// because its memory is directly mapped to the VESA display device's underlying physical memory.
pub fn init_framebuffer<P: Pixel>() -> Result<Framebuffer<P>, &'static str> {
    // get the graphic mode information
    let vesa_display_phys_start: PhysicalAddress;
    let buffer_width: usize;
    let buffer_height: usize;
    {
        let graphic_info = multicore_bringup::GRAPHIC_INFO.lock();
        info!("Using graphical framebuffer, {} x {}, at paddr {:#X}", graphic_info.width, graphic_info.height, graphic_info.physical_address);
        if graphic_info.physical_address == 0 {
            return Err("Failed to get graphic mode information!");
        }
        vesa_display_phys_start = PhysicalAddress::new(graphic_info.physical_address as usize)
            .ok_or("Graphic mode physical address was invalid")?;
        buffer_width = graphic_info.width as usize;
        buffer_height = graphic_info.height as usize;
    };

    // create and return the final framebuffer
    let framebuffer = Framebuffer::new(buffer_width, buffer_height, Some(vesa_display_phys_start))?;
    Ok(framebuffer)
}


/// Starts the first applications that run in Theseus 
/// by creating a new "default" application namespace
/// and spawning the first application `Task`(s). 
/// 
/// Currently this only spawns a shell (terminal),
/// but in the future it could spawn a fuller desktop environment. 
/// 
/// Kernel initialization routines should be complete before invoking this. 
pub fn start(key_consumer: Queue<Event>, mouse_consumer: Queue<Event>) -> Result<(), &'static str> {
    let final_framebuffer: Framebuffer<AlphaPixel> = init_framebuffer()?;
    window_protocol::init(final_framebuffer), key_consumer, mouse_consumer);

    let new_app_ns = mod_mgmt::create_application_namespace(None)?;
    // NOTE: see crate-level docs and note in this crate's `Cargo.toml`.
    let (app_file, _ns) = CrateNamespace::get_crate_object_file_starting_with(
        &new_app_ns, 
        FIRST_APPLICATION_CRATE_NAME,
    ).ok_or("Couldn't find first application in default app namespace")?;

    let path = Path::new(app_file.lock().get_absolute_path());
    info!("Starting first application: crate at {:?}", path);

    //window_manager::init(final_framebuffer, key_consumer, mouse_consumer)?;
    // Spawn the default shell
    spawn::new_application_task_builder(path, Some(new_app_ns))?
        .name("window_manager".to_string())
        .spawn()?;

    Ok(())
}

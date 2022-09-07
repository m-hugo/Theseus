#![no_std]

extern crate spin;
#[macro_use] extern crate log;
#[macro_use] extern crate alloc;
extern crate mpmc;
extern crate event_types;
extern crate compositor;
extern crate framebuffer;
extern crate framebuffer_compositor;
extern crate framebuffer_drawer;
extern crate keycodes_ascii;
extern crate mod_mgmt;
extern crate mouse_data;
extern crate path;
extern crate scheduler;
extern crate spawn;
extern crate window_inner;
extern crate shapes;
extern crate color;

static GLO =

fn register_wm() -> (Arc<Mutex<Framebuffer<AlphaPixel>>>, Queue<Event>, Queue<Event>){
    myglo = GLO;
    GLO = ?;
    myglo
}

fn register_window() -> (Arc<Mutex<Framebuffer<AlphaPixel>>>, Queue<Event>, Queue<Event>){
    GLO
}

fn init(final_framebuffer, key_consumer, mouse_consumer){
    GLO = (Arc::new(Mutex::new(final_framebuffer)), key_consumer, mouse_consumer);
}


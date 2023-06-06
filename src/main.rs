use smithay::{
    backend::{renderer::glow::GlowRenderer, winit},
    utils::Rectangle,
};
use smithay_client_toolkit::{
    compositor::CompositorState,
    output::OutputState,
    reexports::client::{globals::registry_queue_init, Connection},
    registry::RegistryState,
    seat::SeatState,
    shell::wlr_layer::{Layer, LayerShell},
    shm::{slot::SlotPool, Shm},
};
use smithay_egui::EguiState;
use yarrbar::YarrBar;

fn main() {
    let connection = Connection::connect_to_env().unwrap();

    let (globals, mut event_queue) = registry_queue_init(&connection).unwrap();

    let qh = event_queue.handle();

    let compositor = CompositorState::bind(&globals, &qh).unwrap();

    let layer_shell = LayerShell::bind(&globals, &qh).unwrap();

    let shm = Shm::bind(&globals, &qh).unwrap();

    let surface = compositor.create_surface(&qh);

    let layer = layer_shell.create_layer_surface(&qh, surface, Layer::Top, Some("YarrBar"), None);

    let pool = SlotPool::new(256 * 256 * 4, &shm).expect("create pool");

    let (backend, input) = winit::init::<GlowRenderer>().unwrap();

    let egui = EguiState::new(Rectangle::from_loc_and_size(
        (0, 0),
        backend.window_size().physical_size.to_logical(1),
    ));

    let mut yarr_bar = YarrBar {
        registry_state: RegistryState::new(&globals),
        seat_state: SeatState::new(&globals, &qh),
        output_state: OutputState::new(&globals, &qh),
        shm,
        exit: false,
        first_configure: true,
        pool,
        width: 256,
        height: 50,
        shift: None,
        layer,
        keyboard: None,
        keyboard_focus: false,
        pointer: None,
        ui_state: egui,
        backend,
        input,
    };

    loop {
        event_queue.blocking_dispatch(&mut yarr_bar).unwrap();

        if yarr_bar.exit {
            println!("exiting yarrbar");
            break;
        }
    }
}

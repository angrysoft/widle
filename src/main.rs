use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::{wl_registry, wl_seat},
};
use wayland_protocols::ext::idle_notify::v1::client::{
    ext_idle_notification_v1::{self, ExtIdleNotificationV1},
    ext_idle_notifier_v1::ExtIdleNotifierV1,
};

struct AppData {
    idle_notifier: Option<ExtIdleNotifierV1>,
    seat: Option<wl_seat::WlSeat>,
}

fn main() {
    let conn = Connection::connect_to_env().expect("Failed to connect to Wayland compositor");
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let mut state = AppData {
        idle_notifier: None,
        seat: None,
    };

    // 1. Get the registry and find our globals
    let _registry = conn.display().get_registry(&qh, ());

    event_queue.blocking_dispatch(&mut state).unwrap();

    if let (Some(notifier), Some(seat)) = (state.idle_notifier.as_ref(), state.seat.as_ref()) {
        println!("Protocol found! Setting 5-second idle timeout...");

        // Create the notification object
        notifier.get_idle_notification(5000, seat, &qh, ());
    } else {
        panic!("Your compositor does not support ext-idle-notify-v1 or wl_seat.");
    }

    loop {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }
}

// --- Dispatch Implementations ---

// Handle the Idle Notification Events
impl Dispatch<ExtIdleNotificationV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &ExtIdleNotificationV1,
        event: ext_idle_notification_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            ext_idle_notification_v1::Event::Idled => {
                println!("--- IDLE: User has been away for 5 seconds ---")
            }
            ext_idle_notification_v1::Event::Resumed => println!("--- ACTIVE: User is back! ---"),
            _ => (),
        }
    }
}

// Boilerplate: Handle Global Registration
impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        proxy: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name, interface, ..
        } = event
        {
            match &interface[..] {
                "wl_seat" => {
                    state.seat = Some(proxy.bind::<wl_seat::WlSeat, _, _>(name, 1, qh, ()));
                }
                "ext_idle_notifier_v1" => {
                    state.idle_notifier =
                        Some(proxy.bind::<ExtIdleNotifierV1, _, _>(name, 1, qh, ()));
                }
                _ => {}
            }
        }
    }
}

// Boilerplate: Needed for Dispatch traits
impl Dispatch<wl_seat::WlSeat, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &wl_seat::WlSeat,
        _: wl_seat::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}
impl Dispatch<ExtIdleNotifierV1, ()> for AppData {
    fn event(
        _: &mut Self,
        _: &ExtIdleNotifierV1,
        _: <ExtIdleNotifierV1 as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

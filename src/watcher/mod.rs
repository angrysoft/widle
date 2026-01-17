use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::{wl_registry, wl_seat},
};
use wayland_protocols::ext::idle_notify::v1::client::{
    ext_idle_notification_v1::{self, ExtIdleNotificationV1},
    ext_idle_notifier_v1::ExtIdleNotifierV1,
};

mod signal;
use signal::Signal;


pub struct Watcher {
    conn: Connection,
    event_queue: wayland_client::EventQueue<State>,
    qh: QueueHandle<State>,
    timeout: u32,
}

struct State {
    idle_notifier: Option<ExtIdleNotifierV1>,
    seat: Option<wl_seat::WlSeat>,
    signal: Option<Signal>,
}

impl Watcher {
    pub fn new(idle_timeout: u32) -> Self {
        let conn = Connection::connect_to_env().expect("Failed to connect to Wayland compositor");
        let event_queue = conn.new_event_queue();
        let qh = event_queue.handle();

        Watcher {
            conn,
            event_queue,
            qh,
            timeout: idle_timeout,
        }
    }

    pub fn run(&mut self) {
        let signal = match Signal::new() {
            Ok(sig) => {
                println!("D-Bus connection established");
                Some(sig)
            }
            Err(e) => {
                eprintln!(
                    "Warning: Could not connect to D-Bus ({}). Idle hint and lock features will be disabled.",
                    e
                );
                None
            }
        };

        let mut state = State {
            idle_notifier: None,
            seat: None,
            signal,
        };
        // 1. Get the registry and find our globals
        let _registry = self.conn.display().get_registry(&self.qh, ());

        self.event_queue.blocking_dispatch(&mut state).unwrap();

        if let (Some(notifier), Some(seat)) = (state.idle_notifier.as_ref(), state.seat.as_ref()) {
            println!("Protocol found! Setting 5-second idle timeout...");

            // Create the notification object
            notifier.get_idle_notification(self.timeout, seat, &self.qh, ());
        } else {
            panic!("Your compositor does not support ext-idle-notify-v1 or wl_seat.");
        }

        loop {
            self.event_queue.blocking_dispatch(&mut state).unwrap();
        }
    }
}

// Handle the Idle Notification Events
impl Dispatch<ExtIdleNotificationV1, ()> for State {
    fn event(
        state: &mut Self,
        _proxy: &ExtIdleNotificationV1,
        event: ext_idle_notification_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            ext_idle_notification_v1::Event::Idled => {
                println!("--- IDLE: User has been away ---");
                if let Some(signal) = &state.signal {
                    if let Err(e) = signal.set_idle_hint(true) {
                        eprintln!("Warning: Failed to set idle hint: {}", e);
                    }
                    if let Err(e) = signal.lock() {
                        eprintln!("Warning: Failed to lock session: {}", e);
                    }
                }
            }
            ext_idle_notification_v1::Event::Resumed => {
                println!("--- ACTIVE: User is back! ---");
                if let Some(signal) = &state.signal {
                    if let Err(e) = signal.set_idle_hint(false) {
                        eprintln!("Warning: Failed to set idle hint: {}", e);
                    }
                }
            }
            _ => (),
        }
    }
}

// Boilerplate: Handle Global Registration
impl Dispatch<wl_registry::WlRegistry, ()> for State {
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
impl Dispatch<wl_seat::WlSeat, ()> for State {
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
impl Dispatch<ExtIdleNotifierV1, ()> for State {
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

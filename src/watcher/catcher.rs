use zbus::{Result, blocking::Connection, proxy};

#[proxy(
    interface = "org.freedesktop.login1.Session",
    default_service = "org.freedesktop.login1"
)]
trait Session {
    // This defines the signal we want to listen for
    #[proxy(signal)]
    fn lock(&self) -> Result<()>;
}

pub struct Catcher {}

impl Catcher {
    pub fn new() -> Self {
        let connection = Connection::system()?;
        let proxy = SessionProxyBlocking::builder(&connection)
            .path("/org/freedesktop/login1/session/auto")?
            .build()?;

        println!("Listening for Lock signals...");

        // 3. Create an iterator for the 'Lock' signal
        let mut lock_signals = proxy.receive_lock()?;

        // 4. This loop will block and wait for the signal
        for _ in lock_signals {
            println!("Lock signal received! Executing locker program...");

            // Example: Execute a locker program
            // let _ = std::process::Command::new("i3lock")
            //     .arg("-c")
            //     .arg("000000")
            //     .spawn();
        }

        Catcher {}
    }
}

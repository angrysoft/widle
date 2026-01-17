use zbus::{Result, blocking::Connection};
mod catcher;
use Catcher;
pub struct Signal {
    conn: Connection,
}

impl Signal {
    pub fn new() -> Result<Self> {
        let connection = Connection::system()?;
        Ok(Signal { conn: connection })
    }

    pub fn watch(&self) -> Result<()> {
        
        Ok(())
    }

    pub fn set_idle_hint(&self, idle: bool) -> Result<()> {
        self.conn.call_method(
            Some("org.freedesktop.login1"),
            "/org/freedesktop/login1/session/auto",
            Some("org.freedesktop.login1.Session"),
            "SetIdleHint",
            &(idle),
        )?;
        Ok(())
    }

    pub fn lock(&self) -> Result<()> {
        self.conn.call_method(
            Some("org.freedesktop.login1"),
            "/org/freedesktop/login1/session/auto",
            Some("org.freedesktop.login1.Session"),
            "Lock",
            &(),
        )?;
        Ok(())
    }
}

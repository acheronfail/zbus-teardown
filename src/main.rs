use std::time::Duration;

use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use zbus::dbus_proxy;
use zbus::Connection;

// NetworkManager zbus
#[dbus_proxy(
    default_path = "/org/freedesktop/NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager",
    gen_blocking = true
)]
pub trait NetworkManager {
    #[dbus_proxy(signal)]
    fn state_changed(&self) -> zbus::Result<()>;
}

fn main() {
    // create single-threaded runtime
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // run everything within a local set
    tokio::task::LocalSet::new().block_on(&runtime, async_main());
}

async fn async_main() {
    // cancellation token - used to stop the runtime
    let cancel = CancellationToken::new();

    // after a timeout, cancel the runtime
    tokio::task::spawn_local({
        let cancel = cancel.clone();
        async move {
            sleep(Duration::from_secs(1)).await;
            cancel.cancel();
        }
    });

    match std::env::args().skip(1).next().unwrap().as_str() {
        // NOTE: this one panics on drop
        "spawn_local" => {
            tokio::task::spawn_local(spawn_task());
            cancel.cancelled().await;
        }
        // NOTE: this one *does not panic* on drop
        "spawn" => {
            tokio::task::spawn(spawn_task());
            cancel.cancelled().await;
        }
        // NOTE: this one *does not panic* on drop
        "select" => {
            tokio::select! {
                _ = spawn_task() => {},
                _ = cancel.cancelled() => {},
            };
        }
        _ => {
            eprintln!("Pass either 'select', 'spawn' or 'spawn_local'");
            std::process::exit(1);
        }
    };
}

// explicitly logs when dropped so we can confirm when the drop occurs
struct Foo;
impl Drop for Foo {
    fn drop(&mut self) {
        eprintln!("dropping");
    }
}

async fn spawn_task() {
    let connection = Connection::system().await.unwrap();
    let nm_proxy = NetworkManagerProxy::new(&connection).await.unwrap();
    let _stream = nm_proxy.receive_state_changed().await.unwrap();
    let _foo = Foo;

    eprintln!("zbus stream created");

    // used to ensure this task never completes
    futures::future::pending::<()>().await;
}

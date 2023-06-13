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
    gen_blocking = false
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

    // create a new task to listen to network manager changes via zbus
    tokio::task::spawn_local(async move {
        let connection = Connection::system().await.unwrap();
        let nm_proxy = NetworkManagerProxy::new(&connection).await.unwrap();
        let _stream = nm_proxy.receive_state_changed().await.unwrap();

        // used to ensure this task never completes
        futures::future::pending::<()>().await;
    });

    // wait for runtime to be cancelled
    cancel.cancelled().await;
}

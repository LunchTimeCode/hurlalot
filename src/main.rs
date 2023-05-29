mod app;
mod caller;
mod mains;

#[cfg(target_os = "linux")]
fn main() {
    use std::time::Duration;

    use tokio::runtime::Runtime;

    let rt = Runtime::new().expect("Unable to create Runtime");

    let _enter = rt.enter();

    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });
    let _ = mains::linux::linux();
}

#[cfg(target_os = "windows")]
fn main() {
    mains::windows::windows();
}

#[cfg(target_os = "macos")]
fn main() {
    mains::mac::mac();
}

use ipc_channel::ipc;

fn main() {
    let (tx, rx): (ipc::IpcSender<i32>, ipc::IpcReceiver<i32>) = ipc::channel().unwrap();
    println!("Inside webview");
    
}

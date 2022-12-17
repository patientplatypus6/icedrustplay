use std::process::Command;
use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
use std::env;
use serde::*;
use std::{thread, time};

fn main() {

    let datavec = vec![("Peter".to_string(), "36".to_string())];

    println!("Inside Process handler Edited");

    let mut server_args = [
        ("server0", "rungui"),
        ("server1", "runwebview")
    ];


    let (server0, server_name0) = IpcOneShotServer::<Vec<(String, String)>>::new().unwrap();

    println!("server_name0: {:?}", server_name0);

    // let tx = IpcSender::<Vec<(String, String)>>::connect(server_name0.clone()).unwrap();

    // println!("Value of tx is: {:?}", tx);    

    // let wait_time = time::Duration::from_millis(5000);
    // thread::sleep(wait_time);

    let guiserver = spawn_server(
                    "/Users/peterweyand/Code/rustprojects/project1.1/project1_1/process_handler/src/rungui.sh".to_string(), 
                    // &server_args
                    &server_name0
                );


    // let (sub_tx, sub_rx) = ipc::channel::<Vec<(String, String)>>().unwrap();



    // let (tx, rx) = ipc::channel::<Vec<(String, String)>>().unwrap();

    // println!("value of tx and rx, {:?}, {:?}", tx, rx);

    // let (embedded_tx, embedded_rx) = ipc::channel().unwrap();
    // // Send the IpcReceiver
    // tx.send(embedded_rx).unwrap();
    // // Receive the sent IpcReceiver
    // let received_rx = rx.recv().unwrap();
    // // Receive any data sent to the received IpcReceiver
    // let rx_data = received_rx.recv().unwrap();



    // let (server1, server_name1) = IpcOneShotServer::<Vec<(String, String)>>::new().unwrap();

    // let (txph, rxph): (IpcSender<Vec<(String, String)>>, IpcReceiver<Vec<(String, String)>>) = ipc::channel().unwrap();

    // println!("sanity check0");
    // // println!("server0: {:?}", server0);
    // println!("server_name0: {:?}", server_name0);
    // let tx0 = IpcSender::connect(server_name0).unwrap();
    
    // tx0.send(txph).unwrap();
    // rxph.recv().unwrap();
    // let tx1: IpcSender<Vec<(String, String)>> = IpcSender::connect(server_name1).unwrap();
    // tx1.send(datavec).unwrap();


    // println!("value of testing: {:?}", testing);
    println!("sanity check");
    println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&");
    // println!("value of data: {:?}", data);
    println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&");
    // WORKS

    // let (server, server_name) = IpcOneShotServer::<Vec<(String, String)>>::new().unwrap();
    // let person = vec![("Patrick Walton".to_string(), 29.to_string())];
    // let tx: IpcSender<Vec<(String, String)>> = IpcSender::connect(server_name).unwrap();
    // tx.send(person).unwrap();
    // let (_, data): (_, Vec<(String, String)>) = server.accept().unwrap();
    // println!("value of data: {:?}", data);

    // WORKS

    // tx.send(vec![0x10, 0x11, 0x12, 0x13]).unwrap();
    // let (_, data): (_, Vec<u8>) = server.accept().unwrap();
    // assert_eq!(data, vec![0x10, 0x11, 0x12, 0x13]);

    // let server0_name = get_channel_name_arg("server0", &server_args);
    // let server1_name = get_channel_name_arg("server1", &server_args);

    // let (server, server_name) = IpcOneShotServer::new().unwrap();

    // let (tx1, rx1): (IpcSender<Person>, IpcReceiver<Person>) = ipc::channel().unwrap();

    // let tx0 = IpcSender::connect(server0_name).unwrap();



    // let (server, server_name) = IpcOneShotServer::new().unwrap();
    // let tx: IpcSender<Vec<u8>> = IpcSender::connect(server_name).unwrap();

    // tx.send(vec![0x10, 0x11, 0x12, 0x13]).unwrap();
    // let (_, data): (_, Vec<u8>) = server.accept().unwrap();
    // let asserteq = assert_eq!(data, vec![0x10, 0x11, 0x12, 0x13]);
    
    // println!("value of data: {:?}", data);
    // println!("value of asserteq {:?}", asserteq);






    // let (tx1, rx1): (IpcSender<Vec<(std::string::String, std::string::String)>>, IpcReceiver<Vec<(std::string::String, std::string::String)>>) = ipc::channel().unwrap();
    // let tx0 = IpcSender::connect(name).unwrap();
    // tx0.send(tx1).unwrap();
    
    // let (_, tx1): (_, IpcSender<Vec<(std::string::String, std::string::String)>>) = server.accept().unwrap();
    // tx1.send(datavec).unwrap();
    
    // let data = rx1.recv().unwrap();
    // assert_eq!(data, datavec);

    // let ("server1", "runwebview") = IpcOneShotServer::new().unwrap();

    // let tx: IpcSender<Person> = IpcSender::connect("run_gui").unwrap();
    
    // tx.send().unwrap();
    // let (_, data): (_, Vec<u8>) = server.accept().unwrap();
    // assert_eq!(data, vec![0x10, 0x11, 0x12, 0x13]);




    // let (txph, rxph): (ipc::IpcSender<Person>, ipc::IpcReceiver<Person>) = ipc::channel().unwrap();

    // let tx0 = ipc::IpcSender::connect(server0_name).unwrap();
    // tx0.send(person.clone()).unwrap();
    // let received_data = rxph.recv().unwrap();
    // println!("value of received_data: {:?}", received_data);
    // let tx1: IpcSender<Person> = IpcSender::connect(server1_name).unwrap();
    // tx2.send(person.clone()).unwrap();
    
}

pub fn get_channel_name_arg(name: &str, server_args: &[(&str, &str)]) -> String{

    let mut returnstr = "";

    for server in server_args{
        println!("value of server: {:?}", server.1);
        if server.0 == name{
            returnstr = server.1;
        }
    }

    returnstr.to_string()

}

// pub fn get_channel_name_arg(which: &str) -> Option<String> {

//     for arg in env::args() {
//         println!("value of arg in args: {:?}", arg);
//         let arg_str = &*format!("channel_name-{}:", which);
//         if arg.starts_with(arg_str) {
//             return Some(arg[arg_str.len()..].to_owned());
//         }
//     }
//     None
// }

fn simple() {
    let person = ("Patrick Walton".to_owned(), 29);
    let (tx, rx) = ipc::channel().unwrap();
    tx.send(person.clone()).unwrap();
    let received_person = rx.recv().unwrap();
    assert_eq!(person, received_person);
    drop(tx);
    match rx.recv().unwrap_err() {
        ipc::IpcError::Disconnected => (),
        e => panic!("expected disconnected error, got {:?}", e),
    }
}

pub fn spawn_server(address:String, test_name: &str) -> std::process::Child {
    Command::new(address) 
        .arg(test_name)
        .spawn()
        .expect("failed to execute server process")
}
use std::net::{TcpStream};
use std::io::{BufReader, BufRead};
use std::io::Write;
use core::{AlgoType,TypeMessage,tcp_to_typemessage,typemessage_to_tcp,Sorter,BubbleSorter,MergeSorter,InsertionSorter};


fn main() -> std::io::Result<()> {
    let port = std::env::args().nth(3).expect("aucun port indiqué");
    let full_port = format!("127.0.0.1:{}", port);
    let mut name_client = std::env::args().nth(1).expect("aucun nom indiqué");
    let name_client2 = std::env::args().nth(1).unwrap();
    let typ_algo_string =  std::env::args().nth(2).expect("aucun nom indiqué");
    let typ_algo = match typ_algo_string.as_str() {
        "BUBBLE" => AlgoType::Bubblesort,
        "INSERTION" => AlgoType::Insertionsort,
        "MERGE" =>AlgoType::Mergesort,
        _ => AlgoType::Bubblesort,
    };
    let mut msg_to_send : String;
    name_client = format!("CONNECT|{}|{}",name_client,typ_algo_string);
    let mut name_client_byte = name_client.into_bytes();
    name_client_byte.push(b'\n');
    let mut stream = TcpStream::connect(&full_port)?;
    stream.write_all(&name_client_byte).unwrap();
    let stream_lecture = stream.try_clone().unwrap();
    let mut stream_right = stream.try_clone().unwrap();
    let mut reader = BufReader::new(&stream_lecture);
    loop{
        let mut message = String::new();
        match reader.read_line(&mut message) {
            Ok(0) => break,
            Ok(_) => { 
                if message.contains("TAB") {
                    let msgtype: TypeMessage  = tcp_to_typemessage(message).unwrap();
                    let stat_msg = match msgtype {
                        TypeMessage::Tab(mut vec) => { 
                            match typ_algo {
                                AlgoType::Bubblesort => BubbleSorter.sorting(&mut vec, &name_client2),
                                AlgoType::Insertionsort => InsertionSorter.sorting(&mut vec, &name_client2),
                                AlgoType::Mergesort => MergeSorter.sorting(&mut vec, &name_client2)
                            }
                        }
                        _ => { continue }
                    };
                    let msgtypesend =TypeMessage::Resultmessage((stat_msg.result.0,stat_msg.result.1));
                    msg_to_send = typemessage_to_tcp(&msgtypesend).unwrap();
                    stream_right.write_all(msg_to_send.clone().as_bytes()).unwrap();
                } 
                else {} 
            },
            Err(_) => break,
        } 
    };
    Ok(())
}
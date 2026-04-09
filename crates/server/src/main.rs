use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead};
use std::sync::Mutex;
use std::sync::Arc;
use std::io::Write;
use std::{thread, time};
use core::{AlgoType,StatsMessage,TypeMessage,typemessage_to_tcp};




/// fonction qui gère la connexion d'un client
fn handle_worker(stream: TcpStream, liste: Arc<Mutex<Vec<(StatsMessage, TcpStream)>>>) {
    let mut reader = BufReader::new(&stream);
    let mut ligne_name = String::new();
    reader.read_line(&mut ligne_name).unwrap();
    let groups = ligne_name.split('|').collect::<Vec<&str>>();
    let nom = groups[1].to_string();
    let algo_type_string = groups[2].trim();
    let typ_algo = match algo_type_string {
        "BUBBLE" => AlgoType::Bubblesort,
        "INSERTION" => AlgoType::Insertionsort,
        "MERGE" =>AlgoType::Mergesort,
        _ => AlgoType::Bubblesort,
    };
    let new_statmsg: StatsMessage = StatsMessage{name:nom.clone(),type_algo:typ_algo,result:(0 as u128,0 as u64)};

    let stream_ecriture = stream.try_clone().unwrap();
    {
    let mut guard = liste.lock().unwrap();
    guard.push((new_statmsg.clone(), stream_ecriture));
    } 

    loop {        // 1. lire le message de ce client
        let mut message = String::new();
        match reader.read_line(&mut message) {
            Ok(0)  => break,
            Ok(_)  => { 
                        // 2. locker la liste
                        let mut guard = liste.lock().unwrap();
                        let groups = message.split('|').collect::<Vec<&str>>();
                        if groups[0]=="RESULT"{
                            let time : u128 = groups[1].parse().unwrap();
                            let count: u64 = groups[2].trim().parse().unwrap(); 
                            if let Some((stat, _)) = guard.iter_mut().find(|(s, _)| s.name == nom.clone()) {
                                stat.result = (time, count);
                            }
                        }
                        let mut vec_result = Vec::new();
                        // 3. envoyer a tout le monde
                        for (statmsg, _stream_client) in guard.iter_mut() {
                            if statmsg.name != "Dash" {
                                vec_result.push(statmsg.clone());
                            }
                        } 
                        for (_statmsg, stream_client) in guard.iter_mut() {
                            stream_client.write_all(typemessage_to_tcp(&TypeMessage::Stats(vec_result.clone())).unwrap().as_bytes()).unwrap();
                        }
                    }
            Err(_) => break,
        }
        
    }

    {
    let mut guard = liste.lock().unwrap();
    guard.retain(|(statmsg, _)| statmsg.name != nom.clone() );
    } 
}



fn main() -> std::io::Result<()> {
    let port = std::env::args().nth(1).expect("aucun port indiqué");
    let taille_vecteur : i32 = std::env::args().nth(2).expect("aucun port indiqué").parse().unwrap();
    let full_port = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&full_port)?;
    let liste_partage : Arc<Mutex<Vec<(StatsMessage, TcpStream)>>> = Arc::new(Mutex::new(Vec::new()));
    let liste_clone = Arc::clone(&liste_partage);
    std::thread::spawn(move || {
    loop {
        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
        let vec = create_vector(taille_vecteur);
        let msg_type = TypeMessage::Tab(vec);
        let msg_type_sting = typemessage_to_tcp(&msg_type);
        {
        let mut guard = liste_partage.lock().unwrap();
        for (_statmsg, stream_client) in guard.iter_mut(){
             stream_client.write_all(msg_type_sting.clone().unwrap().as_bytes()).unwrap();
            }
        } 
        // envoyer à tous les workers dans liste_clone
    }
    });
    
    for stream in listener.incoming() {
        let liste_clone_2 = Arc::clone(&liste_clone);
        std::thread::spawn(move || {
            handle_worker(stream.unwrap(), liste_clone_2);
        });
    }


    

    Ok(())
}


/// Construit un vecteur aléatoire de u64 avec le nombre d'élément définis par les paramètre 
fn create_vector(nb: i32) -> Vec<u64> {
    let mut seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
    let v = vec![0 as u64; nb as usize];
    v.into_iter().map(|_| find_random(&mut seed)).collect()
}

/// Trouve un élément random grâce à une seed 
fn find_random(seed: &mut u128) -> u64 {
    let a = 1664525;
    let c = 1013904223;
    let m = 4294967296;  // 2^32  ;
    *seed = (*seed * a + c) % m;
    let number = *seed % 1000.0 as u128;
    number as u64
}
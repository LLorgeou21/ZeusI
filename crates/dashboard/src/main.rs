use std::net::{TcpStream};
use std::io::{BufReader, BufRead};
use std::io::Write;
use eframe::egui;
use egui::Frame;
use core::{StatsMessage,TypeMessage,tcp_to_typemessage};



fn main()  -> std::io::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel::<TypeMessage>();
    let port = std::env::args().nth(1).expect("aucun port indiqué");
    let full_port = format!("127.0.0.1:{}", port);
    let mut stream = TcpStream::connect(&full_port)?;
    let stream_lecture = stream.try_clone().unwrap();

    std::thread::spawn(move || {
        let mut msg_connect = format!("CONNECT|Dash|Dash").into_bytes();
        msg_connect.push(b'\n');
        stream.write_all(&msg_connect).unwrap();
        let mut ligne_name = String::new();
        let mut reader = BufReader::new(&stream_lecture);
        loop{
            reader.read_line(&mut ligne_name).unwrap();
            if ligne_name.contains("STAT"){
                let type_msg = tcp_to_typemessage(ligne_name.clone()).unwrap();
                tx.send(type_msg).unwrap();
            }
            ligne_name.clear();
        }  
    }
    );

    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("MyApp", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc,rx)))));
    Ok(())
}



struct MyEguiApp {
    etat_stat: Vec<StatsMessage>,
    rx : std::sync::mpsc::Receiver<TypeMessage>
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>, rx_new : std::sync::mpsc::Receiver<TypeMessage>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
            Self {
                etat_stat: Vec::new(),
                rx :  rx_new
            }
        }
    }

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            match self.rx.try_recv() {
                Ok(message) => {
                    self.etat_stat = match message {
                        TypeMessage::Stats(vec) => vec,
                        _ => Vec::new()
                    };
                },
                Err(_) => {}
            }
            tri_tri(&mut self.etat_stat);
            ui.columns(2,|cols|{
                cols[0].vertical_centered(|ui| {
                    ui.label(egui::RichText::new("ZeusI est un outil de benchmarking distribue qui permet de visualiser et comparer en temps reel les performances d'algorithmes de tri s'executant sur differentes machines en reseau.
                                    Connecte plusieurs workers avec des algorithmes differents et observe en direct lequel trie le plus rapidement.\n \n \n
                                    ZeusI is a distributed benchmarking tool that allows you to visualize and compare in real time the performance of sorting algorithms running on different machines over a network.
                                    Connect several workers with different algorithms and watch live 
                                    which one sorts the fastest."
                                )
                                .color(egui::Color32::from_rgb(0, 0, 0)));
                });
                if self.etat_stat.is_empty() { return; }
                let hauteur_carte = cols[1].available_height() / self.etat_stat.len() as f32;
                for data in self.etat_stat.iter(){
                    Frame::new()
                        .fill(egui::Color32::from_rgb(180, 200, 240))
                        .outer_margin(egui::Margin::same(2))
                        .show(&mut cols[1], |ui| {
                            ui.set_min_height(hauteur_carte);
                            ui.set_height(hauteur_carte);
                            ui.centered_and_justified(|ui| {
                                ui.label(egui::RichText::new(format!("Worker : \t{}\nMéthode de tri : \t{}\nTemps pour trier : \t{} µs \nComparaison totale : \t{}", data.name, data.type_algo, data.result.0, data.result.1)
                                ).size(18.0)
                                .color(egui::Color32::from_rgb(0, 0, 0)));
                            });
                            
                        });
                }
            });
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(2));
        });
    }
}


fn tri_tri(vec : &mut Vec<StatsMessage>){
    vec.sort_by(|a,b| a.result.0.cmp(&b.result.0));
}
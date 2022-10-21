use std::io;
#[derive(Debug)]
pub struct Filter{
    ip_source: String,
    ip_dest: String,
    port_source: String,
    port_dest: String,
    transport_protocol: String,
}
impl Filter {
    pub fn new() -> Self {
        return Filter{ip_source: "".to_string(),
                      ip_dest: "".to_string(),
                      port_source: "".to_string(),
                      port_dest: "".to_string(),
                      transport_protocol: "".to_string()};
    }
    /*
    pub fn set_ip_source(self, ip_source: String) -> Self {
        return Filter{ip_source: ip_source, ip_dest: self.ip_dest};
    }

    
    pub fn set_ip_dest(self, ip_dest: String) -> Self {
        return Filter{ip_source: self.ip_source, ip_dest: ip_dest};
    }
    */
    

}

pub fn print_index() -> () {
    print!("{}[2J", 27 as char);
    println!("Packet Swiffer v.1.0");
    println!("Author: Barletta Francesco Pio, Cosimo Simone, Ferla Damiano");
    println!("Politecnico di Torino - All Rights Deserved");
    println!("\n");
    
    println!("1.\t Start Sniffing");
    println!("2.\t Set Filters");
    println!("\n\n");
    println!("While sniffing, press P to stop/resume");
}

/*
Da sistemare la struct da popolare
*/
pub fn menu() -> Filter {
    let mut filters = Filter::new();
    loop {
        print_index();
        let mut buffer = String::new();
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        match buffer.as_str().trim() {
            "1" => {
                break;
             }
            "2" => {
                filters = print_filter();
            }
            _ => {
                println!("Wrong command.");
            }
        }
    }
    return filters;    
}

pub fn filter_menu() -> () {
    print!("{}[2J", 27 as char);
    println!("Filter settings:");
    println!("\n");
    println!("1.\t Filtra per indirizzo IP sorgente");
    println!("2.\t Filtra per indirizzo IP destinazione");
    println!("3.\t Filtra per porta sorgente");
    println!("4.\t Filtra per porta destinazione");
    println!("5.\t Filtra per protocollo di trasporto");
    println!("0.\t Back to menu");
}

pub fn print_filter() -> Filter{
    let mut buffer = String::new();

    let mut vec_ip_source: Vec<String> = Vec::new();
    let mut ip_source = String::new();

    let mut vec_ip_dest: Vec<String> = Vec::new();
    let mut ip_dest = String::new();

    let mut vec_port_source: Vec<String> = Vec::new();
    let mut port_source = String::new();

    let mut vec_port_dest: Vec<String> = Vec::new();
    let mut port_dest = String::new();

    let mut vec_transport_protocol: Vec<String> = Vec::new();
    let mut transport_protocol = String::new();

    let mut filter = Filter::new();
    loop {
        filter_menu();
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        match buffer.as_str().trim() {
            "1" => {
                vec_ip_source.push(filter_ip_source());
                buffer.clear();
            }
            "2" => {
                vec_ip_dest.push(filter_ip_dest());
                buffer.clear();
            }
            "3" => {
                vec_port_source.push(filter_port_source());
                buffer.clear();
            }
            "4" => {
                vec_port_dest.push(filter_port_dest());
                buffer.clear();
            }
            "5" => {
                vec_transport_protocol.push(filter_transport_protocol());
                buffer.clear();
            }
            "0" => {
                if !vec_ip_source.is_empty() {
                    let mut last = vec_ip_source.last().unwrap().to_string();
                    for x in vec_ip_source{
                        if x == last {
                            ip_source = "src host".to_string() + &ip_source.trim().to_string() + &x;
                        }
                        else {
                            ip_source = "src host".to_string() + &ip_source.trim().to_string() + &x + &"or".to_string();
                        }
                    }
                }
                
                if !vec_ip_dest.is_empty() {
                    let mut last = vec_ip_dest.last().unwrap().to_string();
                    for x in vec_ip_dest{
                        if x == last {
                            ip_dest = "dst host".to_string() + &ip_dest.trim().to_string() + &x;
                        }
                        else {
                            ip_dest = "dst host".to_string() + &ip_dest.trim().to_string() + &x + &"or".to_string();
                        }
                    }
                }

                if !vec_port_source.is_empty() {
                    let mut last = vec_port_source.last().unwrap().to_string();
                    for x in vec_port_source{
                        if x == last {
                            port_source = "src port".to_string() + &port_source.trim().to_string() + &x;
                        }
                        else {
                            port_source = "src port".to_string() + &port_source.trim().to_string() + &x + &"or".to_string();
                        }
                    }
                }

                if !vec_port_dest.is_empty() {
                    let mut last = vec_port_dest.last().unwrap().to_string();
                    for x in vec_port_dest{
                        if x == last {
                            port_dest = "dst port".to_string() + &port_dest.trim().to_string() + &x;
                        }
                        else {
                            port_dest = "dst port".to_string() + &port_dest.trim().to_string() + &x + &"or".to_string();
                        }
                    }
                }
                
                if !vec_transport_protocol.is_empty() {
                    let mut last = vec_transport_protocol.last().unwrap().to_string();
                    for x in vec_transport_protocol{
                        if x == last {
                            transport_protocol = "ip proto".to_string() + &transport_protocol.trim().to_string() + &x;
                        }
                        else {
                            transport_protocol = "ip proto".to_string() + &transport_protocol.trim().to_string() + &x + &"or".to_string();
                        }
                    }
                }

                filter = Filter{
                    ip_source: ("src host ".to_string() + &ip_source).trim().to_string(),
                    ip_dest: ("dst host ".to_string() + &ip_dest).trim().to_string(),
                    port_source: ("src port ".to_string() + &port_source).trim().to_string(),
                    port_dest: ("dst port ".to_string() + &port_dest).trim().to_string(),
                    transport_protocol: ("ip proto ".to_string() + &transport_protocol).trim().to_string()
                };
                break;
            }
            _ => {}
        }
    }
    return filter;   
}

pub fn filter_ip_source() -> String {
    print!("{}[2J", 27 as char);
    println!("Filtra per indirizzo IP sorgente: \n");
    println!("Inserisci indirizzo IP sorgente");
    let mut buffer = String::new();
    buffer.clear();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    return buffer;
}

pub fn filter_ip_dest() -> String {
    print!("{}[2J", 27 as char);
    println!("Filtra per indirizzo IP destinazione: \n");
    println!("Inserisci indirizzo IP destinazione");
    let mut buffer = String::new();
    buffer.clear();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    return buffer;
}

pub fn filter_port_source() -> String {
    print!("{}[2J", 27 as char);
    println!("Filtra per porta sorgente: \n");
    println!("Inserisci porta sorgente");
    let mut buffer = String::new();
    buffer.clear();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    return buffer;
}

pub fn filter_port_dest() -> String {
    print!("{}[2J", 27 as char);
    println!("Filtra per porta destinazione: \n");
    println!("Inserisci porta destinazione");
    let mut buffer = String::new();
    buffer.clear();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    return buffer;
}

pub fn filter_transport_protocol() -> String {
    print!("{}[2J", 27 as char);
    println!("Filtra per protocollo di trasporto: \n");
    println!("Inserisci protocollo di trasporto");
    let mut buffer = String::new();
    buffer.clear();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    return buffer;
}

pub fn parse_filter(filter: Filter) -> String {
    // TODO: To discuss

    // Iterate over the struct fields
    // Create a string, concat with && or || 

    // Dummy return
    return "".to_string();
}
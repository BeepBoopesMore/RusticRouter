use core::panic;
use std::fs::read_to_string;
use std::num::{NonZero, ParseIntError};
use std::os::unix::fs::PermissionsExt;
use std::path::Prefix;
use std::ptr::read;
use std::{cell::RefCell, ffi::FromVecWithNulError, os::unix::net::UnixDatagram};
use regex::{Captures, Regex};
use serde::de::value::Error;
use ssh2::DisconnectCode::ProtocolError;
use std::{fs, string, vec};
use std::io::{self, Read};
use std::collections::HashMap;
use ssh2::{File, Session};
use std::net::TcpStream;
use std::process::{Command, Output};
use std::file;




// Just easier to match the patterns ig lol 
enum RegexPatternShowCommands{
   interfaces_ipv4,
   interfaces_ipv6,
   current_routing_protocol,
   cdp_neighbors_simple,
   cdp_neighbors_detailed,
   lldp_neigbors_detailed,
   lldp_neighbors_simple,
   license,
   ip_route_table_ipv4,
   ip_route_table_ipv6,
   dhcp_bindings,
   vlan_info,
   cpu_utilization,
   ospf_database,
   ospf_neighbors,
   bgp_info,
   rip_info,
   memory_utilization,
   processes,
   access_lists,





}





fn get_regex(x:RegexPatternShowCommands,vendor:Vendors) -> String{
   let mut pattern:String;
   match vendor{
      Vendors::Cisco => {
         match x {
            RegexPatternShowCommands::interfaces_ipv4 => pattern = String::from(r""),
            RegexPatternShowCommands::interfaces_ipv6 => pattern = String::from(r""),
            RegexPatternShowCommands::current_routing_protocol=> pattern = String::from(r""),
            RegexPatternShowCommands::cdp_neighbors_detailed => pattern =String::from(r""),
            RegexPatternShowCommands::cdp_neighbors_simple => pattern = String::from(r""),
            RegexPatternShowCommands::lldp_neighbors_simple => pattern = String::from(r"(?m)^(?P<device>\S+)\s+(?P<local_intf>\S+)\s+(?P<hold_time>\d+)\s+(?P<capability>[\w,]+)\s+(?P<port_id>\S+)$"),
            RegexPatternShowCommands::lldp_neigbors_detailed => pattern = String::from(r""),
            RegexPatternShowCommands::license => pattern = String::from(r""),
            RegexPatternShowCommands::ip_route_table_ipv4 => pattern = String::from(r""),
            RegexPatternShowCommands::ip_route_table_ipv6 => pattern = String::from(r"^(?P<code>[A-Z]+)\s+(?P<prefix>[0-9a-fA-F:]+/\d+).+?via\s+(?P<nexthop>[0-9a-fA-F:]+|::)(?:,\s*(?P<interface>[\w\-/\.]+))?"),
            RegexPatternShowCommands::dhcp_bindings => pattern = String::from(r""),
            RegexPatternShowCommands::vlan_info => pattern = String::from(r""),
            RegexPatternShowCommands::cpu_utilization => pattern = String::from(r""),
            RegexPatternShowCommands::ospf_database => pattern = String::from(r""),
            RegexPatternShowCommands::ospf_neighbors=> pattern = String::from(r""),
            RegexPatternShowCommands::bgp_info => pattern = String::from(r""),
            RegexPatternShowCommands::rip_info => pattern = String::from(r""),
            RegexPatternShowCommands::memory_utilization => pattern = String::from(r""),
            RegexPatternShowCommands::processes => pattern = String::from(r""),
            RegexPatternShowCommands::access_lists => pattern = String::from(r""),
            
               
            _ => panic!("s")
         }




      }

      Vendors::Juniper =>{






      }

      _ => panic!("")
   }




   let mut pattern = "".to_string();










   pattern



}















// Prefixes for the stupid Juniper regulations
enum Prefixes {
   Prefix0,
   Prefix1,
   Prefix2,
   Prefix3,
   Prefix4,
   Prefix5,
   Prefix6,
   Prefix7,
   Prefix8,
   Prefix9,
   Prefix10,
   Prefix11,
   Prefix12,
   Prefix13,
   Prefix14,
   Prefix15,
   Prefix16,
   Prefix17,
   Prefix18,
   Prefix19,
   Prefix20,
   Prefix21,
   Prefix22,
   Prefix23,
   Prefix24,
   Prefix25,
   Prefix26,
   Prefix27,
   Prefix28,
   Prefix29,
   Prefix30,
   Prefix31,
   Prefix32,
}

fn get_prefix(prefix: Prefixes) -> String {
   let mask: String;
   match prefix {
      Prefixes::Prefix0 => mask = String::from("0.0.0.0"),
      Prefixes::Prefix1 => mask = String::from("128.0.0.0"),
      Prefixes::Prefix2 => mask = String::from("192.0.0.0"),
      Prefixes::Prefix3 => mask = String::from("224.0.0.0"),
      Prefixes::Prefix4 => mask = String::from("240.0.0.0"),
      Prefixes::Prefix5 => mask = String::from("248.0.0.0"),
      Prefixes::Prefix6 => mask = String::from("252.0.0.0"),
      Prefixes::Prefix7 => mask = String::from("254.0.0.0"),
      Prefixes::Prefix8 => mask = String::from("255.0.0.0"),
      Prefixes::Prefix9 => mask = String::from("255.128.0.0"),
      Prefixes::Prefix10 => mask = String::from("255.192.0.0"),
      Prefixes::Prefix11 => mask = String::from("255.224.0.0"),
      Prefixes::Prefix12 => mask = String::from("255.240.0.0"),
      Prefixes::Prefix13 => mask = String::from("255.248.0.0"),
      Prefixes::Prefix14 => mask = String::from("255.252.0.0"),
      Prefixes::Prefix15 => mask = String::from("255.254.0.0"),
      Prefixes::Prefix16 => mask = String::from("255.255.0.0"),
      Prefixes::Prefix17 => mask = String::from("255.255.128.0"),
      Prefixes::Prefix18 => mask = String::from("255.255.192.0"),
      Prefixes::Prefix19 => mask = String::from("255.255.224.0"),
      Prefixes::Prefix20 => mask = String::from("255.255.240.0"),
      Prefixes::Prefix21 => mask = String::from("255.255.248.0"),
      Prefixes::Prefix22 => mask = String::from("255.255.252.0"),
      Prefixes::Prefix23 => mask = String::from("255.255.254.0"),
      Prefixes::Prefix24 => mask = String::from("255.255.255.0"),
      Prefixes::Prefix25 => mask = String::from("255.255.255.128"),
      Prefixes::Prefix26 => mask = String::from("255.255.255.192"),
      Prefixes::Prefix27 => mask = String::from("255.255.255.224"),
      Prefixes::Prefix28 => mask = String::from("255.255.255.240"),
      Prefixes::Prefix29 => mask = String::from("255.255.255.248"),
      Prefixes::Prefix30 => mask = String::from("255.255.255.252"),
      Prefixes::Prefix31 => mask = String::from("255.255.255.254"),
      Prefixes::Prefix32 => mask = String::from("255.255.255.255"),
   }
   mask
}

// Vendors that I will match
enum Vendors {
   Cisco,
   Juniper,
}

// This will be the processing data using regex
struct Utils {
   // Show commands mostly because the other functions i will do later
   interfaces_ipv4: Option<Vec<String>>,
   interfaces_ipv6: Option<Vec<String>>,
   current_routing_protocol: Option<String>,
   cdp_neighbors_simple: Option<HashMap<String,Vec<String>>>,
   cdp_neighbors_detailed: Option<HashMap<String,Vec<String>>>,
   lldp_neighbors_simple: Option<HashMap<String,Vec<String>>>,
   lldp_neigbors_detailed: Option<Vec<String>>,
   license: Option<String>,
   ip_route_table_ipv4: Option<Vec<String>>,
   ip_route_table_ipv6: Option<Vec<String>>,
   dhcp_bindings: Option<Vec<String>>,
   vlan_info : Option<Vec<String>>,
   cpu_utilization : Option<Vec<String>>,
   ospf_neighbors : Option<Vec<String>>,
   ospf_database : Option<Vec<String>>,
   bgp_info : Option<Vec<String>>,
   rip_info : Option<Vec<String>>,
   memory_utilization:Option<Vec<String>>,
   processes :Option<String>,
   access_lists: Option<Vec<String>>,





   vendor: String,
   output: String,
}

impl Utils {
   fn new(output: String, vendor: String) -> Self {
      Utils {
         interfaces_ipv4: None,
         interfaces_ipv6: None,
         current_routing_protocol: None,
         cdp_neighbors_simple: None,
         cdp_neighbors_detailed: None,
         lldp_neighbors_simple: None,
         lldp_neigbors_detailed: None,
         license: None,
         ip_route_table_ipv4: None,
         ip_route_table_ipv6: None,
         dhcp_bindings:None,
         vlan_info:None,
         cpu_utilization:None,
         ospf_database:None,
         ospf_neighbors:None,
         bgp_info:None,
         rip_info:None,
         memory_utilization:None,
         processes:None,
         access_lists:None,
         vendor,
         output,
      }
   }

   fn interfaces_ipv4(&mut self) {
      if self.interfaces_ipv4.is_none() {
         if self.vendor.to_lowercase() == "cisco" {
            let output = &self.output.as_str();
            let regex_pattern = r"(?m)^(\S+)\s+\S+\s+\S+\s+\S+\s+up\s+up\s*$";
            let re = Regex::new(regex_pattern).unwrap();
            let mut new_interface_array: Vec<String> = vec![];
            for caps in re.captures_iter(output) {
               let the_thing_we_looking_for = caps[1].to_string();
               new_interface_array.push(the_thing_we_looking_for);
            }
            self.interfaces_ipv4 = Some(new_interface_array.clone());
         } else if self.vendor.to_lowercase() == "juniper" {
            let output = &self.output.as_str();
            let regex_pattern =
               r"(?m)^([a-z]{2,3}-\d+/\d+/\d+(?:[:\.]\d+)?)\s+\S+\s+\S+\s+inet\b";
            let re = Regex::new(regex_pattern).unwrap();
            let mut new_interface_array: Vec<String> = vec![];
            for caps in re.captures_iter(output) {
               let the_thing = caps[1].to_string();
               new_interface_array.push(the_thing);
            }
            self.interfaces_ipv4 = Some(new_interface_array).clone();
         }
      }
   }

   fn interfaces_ipv6(&mut self) {
      if self.interfaces_ipv6.is_none() {
         if self.vendor.to_lowercase() == "cisco" {
            let output = &self.output.as_str();
            let regex_pattern = r"^(\S+)\s+";
            let re = Regex::new(regex_pattern).unwrap();
            let mut new_interface_ipv6: Vec<String> = vec![];
            for caps in re.captures_iter(output) {
               let the_ipv6 = caps[1].to_string();
               new_interface_ipv6.push(the_ipv6);
            }
            self.interfaces_ipv6 = Some((new_interface_ipv6).clone());
            println!("{:?}", &new_interface_ipv6);
         } else if self.vendor.to_lowercase() == "juniper" {
           let output = &self.output.as_str();
           let regex_pattern =  get_regex(RegexPatternShowCommands::interfaces_ipv6, Vendors::Juniper);
            let re = Regex::new(&regex_pattern).unwrap();
            let mut new_interface_ipv6: Vec<String> = vec![];
            for caps in re.captures_iter(output) {
               let g = caps[1].to_string();
               println!("{}",g);
               new_interface_ipv6.push(g);
            }
            self.interfaces_ipv6 = Some((new_interface_ipv6).clone());
         //TODO HERE TOO 
         }
      }
   }

   fn current_routing_protocol(&mut self) {
      if self.current_routing_protocol.is_none(){
         if self.vendor.to_lowercase() == "cisco"{
             
         }
         else if self.vendor.to_lowercase() == "juniper"{
             //TODO
         }
      }
   }

   fn cdp_neighbors_simple(&mut self) {
      if self.cdp_neighbors_simple.is_none() {
         if self.vendor.to_lowercase() == "cisco" {
            let output = &self.output.as_str();
            let regex_pattern = r"(?P<DeviceID>\S+)\s+(?P<LocalIntrfce>\S+\s\S+)\s+(?P<Holdtime>\d+)\s+(?P<Capability>[A-Za-z\s]+)\s+(?P<Platform>\S+)\s+(?P<PortID>\S+\s\S+)";
            let re:Regex = Regex::new(regex_pattern).unwrap();
            let mut device_id:Vec<String> = vec![];
            let mut local_interface:Vec<String> = vec![];
            let mut holdtime :Vec<String> = vec![];
            let mut capability:Vec<String> = vec![];
            let mut port_id:Vec<String>=vec![];
            for caps in re.captures_iter(output){
               device_id.push(caps[1].to_string());
               local_interface.push(caps[2].to_string());
               holdtime.push(caps[3].to_string());
               capability.push(caps[4].to_string());
               port_id.push(caps[6].to_string());


      
            }  

            let mut dummy_hashmap:HashMap<String,Vec<String>> =  HashMap::new();
            dummy_hashmap.insert("Device Id".to_string(), device_id);
            dummy_hashmap.insert("Local Interface".to_string(), local_interface);
            dummy_hashmap.insert("Holdtime".to_string(), holdtime);
            dummy_hashmap.insert("Capability".to_string(), capability);
            dummy_hashmap.insert("Port Id ".to_string(), port_id);


           self.cdp_neighbors_simple = Some((dummy_hashmap).clone())
           



            

         }
         else if self.vendor.to_lowercase() == "juniper"{

            panic!("Juniper doesnt  have cdp");
         }
      
      }
   }

   fn cdp_neighbors_detailed(&mut self) {
      if self.cdp_neighbors_detailed.is_none(){
         match self.vendor.to_lowercase().as_str(){
            "cisco" => {
               let output = &self.output.as_str();
               let pattern = "";
               let re = Regex::new(r#"(?xmsi)
    -{20,}\n
    Device\sID:\s*(?P<device>[^\n]+)\n
    .*?IP\saddress:\s*(?P<ip>[^\s]+)\n
    Platform:\s*(?P<platform>[^,]+),\s*
    Capability:\s*(?P<capability>[^\n]+)\n
    Interface:\s*(?P<local_port>[^,]+),\s*
    Port\sID.*?:\s*(?P<remote_port>[^\s]+)\n
    Hold.*?:\s*(?P<holdtime>\d+).*?\n
    (?:Version.*?\n(?P<version>(?s).*?))(?=\n\S|\Z)
"#).unwrap();
               let device_id:Vec<String> = vec![];
               for caps in re.captures_iter(output){
                  println!("{}",&caps["device"].to_string());
                  
               }

               let mut dummy_hashmap:HashMap<String, Vec<String>> = HashMap::new();
               dummy_hashmap.insert("Hello".to_string(),device_id);
   
               self.cdp_neighbors_detailed = Some((dummy_hashmap.clone()))
               


            },
            "juniper" =>{
               panic!("Juniper does not have cdp !")
            },
            _ => println!("Not a right vendor")
         }

         }
      }


   fn lldp_neighbors_simple(&mut self){
      if self.lldp_neighbors_simple.is_none(){
         match self.vendor.to_lowercase().as_str(){
            "cisco" =>{
               let output = &self.output;
               let regex_pattern = get_regex( RegexPatternShowCommands::lldp_neighbors_simple    ,Vendors::Cisco);
               let re = Regex::new(regex_pattern.as_str()).unwrap();
               let mut device_id:Vec<String> =  vec![];
               let local_interface : Vec<String> = vec![];
               let holdtime : Vec<String> = vec![];
               let capability  : Vec<String> = vec![];
               let port_id: Vec<String> =  vec![];

               for caps in re.captures_iter(output){

                  // Push here , TODO 

               }
            


              let mut dummy_hashmap: HashMap<String,Vec<String>> = HashMap::new();
              dummy_hashmap.insert("Device Id ".to_string(), device_id);
              dummy_hashmap.insert("Local Interfaces".to_string(), local_interface);
              dummy_hashmap.insert("Holdtimes".to_string(),holdtime );
              dummy_hashmap.insert("Capability".to_string(), capability);
              dummy_hashmap.insert("Port Id".to_string(),port_id);


              self.lldp_neighbors_simple = Some((dummy_hashmap).clone())
      



               




               // TODO 
            },
            "juniper" => {
               let output = &self.output;
               // TODO 
            }
            _ => panic!("Not a right vendor")
         }
      }
   } 


   fn lldp_neigbors_detailed(&mut self) {

      if self.lldp_neigbors_detailed.is_none(){
         match self.vendor.to_lowercase().as_str(){
            "cisco" =>{
               //todo
            },
            "juniper" => {
               //TODO
            },
            _ => println!("Please mention another vendor as this is not working")
         }
      }

   

   }
   fn ip_route_table_ipv4(&mut self){
      if self.ip_route_table_ipv4.is_none(){
         match self.vendor.to_lowercase().as_str(){
            "cisco" => {
               //todo
            },
            "juniper" => {
               // to do
            },
            _ => println!("Not a right  vendor ")
         }
      }
   }





   fn ip_route_table_ipv6(&mut self){
      if self.ip_route_table_ipv6.is_none(){
         match self.vendor.to_lowercase().as_str(){
            "cisco" => {
               //todo
            },
            "juniper" => {
               //to do
            },
            _ => println!("Not a right vendor ")

         }

      }

   }

   fn  dhcp_bindings(&mut self){
      if self.dhcp_bindings.is_none(){
         match self.vendor.to_lowercase().as_str(){
            "cisco " => {
               //todo 
            }
            
            "juniper" => {
               //todo
            }
            _ => println!("Not a right vendor")
         }
      }
       
   }

   fn vlan_info(&mut self ){
      if self.vlan_info.is_none(){
         match self.vendor.to_lowercase().as_str(){
            "cisco" => {
               //todo
            },
            "juniper" => {
               //todo
            },

            _ => println!("Not a right vendor")



         }
      }
   }

   fn cpu_utilization(&mut self){
      if self.cpu_utilization.is_none(){
         match self.vendor.to_lowercase().as_str(){
            "cisco" => {
               //
            }
            "juniper" => {
               //
            }
            

            _ => println!("wrong vendor")
         }
      }
   }

   fn ospf_neighbors(&mut self){
      if self.ospf_neighbors.is_none(){
         match self.vendor.to_lowercase().as_str(){
            "cisco" => {
               //
            }
            "juniper" => {
               //
            }


            _ => println!("Wrong vendor")



         }


      }



   }

   fn ospf_database(&mut self ){
      if self.ospf_database.is_none(){
         match self.vendor.to_lowercase().as_str(){
            "cisco" => {
               // To do
            
            }

            "juniper" => {
               // TODO 

            }
            _ => println!("Wrong vendor ")

         }

      }
   }









}
            

// This will match behavior like Router, Switch, Server
enum LaterBehavior {
   Router,
   Switch,
   Server,
}

fn choice(choice: LaterBehavior) {
   match choice {

      LaterBehavior::Router => {
         //TODO
      },
      LaterBehavior::Switch => {
         //TODO
      }
      LaterBehavior::Server => {
         //TODO
      }
      _ => panic!("Not a right choice ")
   }
}

// Sending command via ssh2
struct CommandSSH<T: ToString> {
   host_ip: T,
   host_name: T,
   host_password: T,
   output_given_after: Option<String>,
}

impl<T: ToString> CommandSSH<T> {
   pub fn new(host_ip: T, host_name: T, host_password: T) -> Self {
      CommandSSH {
         host_ip,
         host_name,
         host_password,
         output_given_after: None,
      }
   }

   pub fn process_command(&mut self, command: String) {
      if self.output_given_after.is_none() {
         let tcp: TcpStream = TcpStream::connect(self.host_ip.to_string()).unwrap();
         let mut session = Session::new().unwrap();
         session.set_tcp_stream(tcp);
         session.handshake().unwrap();
         let username = &self.host_name;
         let password = &self.host_password;
         session.userauth_password(&username.to_string(), &password.to_string());
         let mut channel = session.channel_session().unwrap();
         let mut string_use = String::new();
         channel.exec(&command).unwrap();
         channel.read_to_string(&mut string_use).unwrap();
         self.output_given_after = Some((string_use));
         channel.wait_close();
      }
   }
}

fn main() {


   let data = String::from("
   -------------------------
Device ID: Switch1
Entry address(es):
  IP address: 10.1.1.2
Platform: cisco WS-C2960X-48FPS-L,  Capabilities: Switch IGMP
Interface: GigabitEthernet1/0/1,  Port ID (outgoing port): GigabitEthernet0/1
Holdtime : 142 sec

Version :
Cisco IOS Software, C2960X Software (C2960X-UNIVERSALK9-M), Version 15.2(2)E6, RELEASE SOFTWARE (fc1)
Technical Support: http://www.cisco.com/techsupport
Copyright (c) 1986-2017 by Cisco Systems, Inc.
Compiled Mon 18-Dec-17 06:03 by prod_rel_team

Advertisement Version: 2
Duplex: full
   
   
   ");

   let mut file_1 = Utils::new(data.to_string(), "cisco".to_lowercase());

   file_1.cdp_neighbors_detailed();
   let g = file_1.cdp_neighbors_detailed();
   println!("{:?}",g)

}

struct Router {
   host_ip: String,
   host_password: String,
   host_username: String,
   host_vendor: String,
   test: Option<String>,
   interfaces_ipv4: Option<Vec<String>>,
   hostname: Option<String>,
}

impl Router {
   pub fn new(
      host_username: String,
      host_ip: String,
      host_password: String,
      host_vendor: String,
   ) -> Self {
      let vendor_list: [String; 2] = ["cisco".to_string(), "juniper".to_string()];
      let lower_case: String = host_vendor.to_lowercase();
      if vendor_list.contains(&lower_case) {
         let ping = Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg(&host_ip)
            .output()
            .expect("Failed to run the ping");
         if ping.status.success() {
            return Router {
               host_ip,
               host_password,
               host_vendor,
               host_username,
               test: None,
               interfaces_ipv4: None,
               hostname: None,
            };
         } else {
            panic!("The Host is down try another one")
         }
      } else {
         panic!("Use another vendor");
      }
   }

   pub fn interfaces_ipv4(&mut self) {
      if self.interfaces_ipv4.is_none() {
         if self.host_vendor.to_lowercase() == "cisco" {
            let command = "enable && show ip interface brief".to_string();
            let mut new_command =
               CommandSSH::new(&self.host_ip, &self.host_username, &self.host_password);
            new_command.process_command(command);
            let output = new_command.output_given_after.unwrap();
            let mut let_made_g = Utils::new(output, "cisco".to_string());
            let_made_g.interfaces_ipv4();
            let interface_double = let_made_g.interfaces_ipv4;
            self.interfaces_ipv4 = Some((interface_double).unwrap());

         }
      }
   }
}

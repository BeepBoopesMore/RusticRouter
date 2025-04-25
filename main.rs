
      
   












use std::num::{NonZero, ParseIntError};
use std::ptr::read;
use std::{cell::RefCell, ffi::FromVecWithNulError, os::unix::net::UnixDatagram};
use regex::{Captures, Regex};
use serde::de::value::Error;
use std::{fs, vec};
use std::io::{self, Read};
use std::collections::HashMap;
use ssh2::Session;
use std::net::TcpStream;
use std::process::Command;




// This will or won't be finished if i have time to import it all from python who knows , the main reason i did it is because Rust has a wonderful UI frame-work..


// Vendors that i will match 
 
enum Vendors{
   cisco,
   juniper
}

// This will be the processing data using regex.. 
struct Utils{
   // Show command
   interfaces_ipv4 : Option<Vec<String>>,
   interfaces_ipv6 : Option<Vec<String>>,
   current_routing_protocol : Option<String>,
   cdp_neighbors_simple :Option<Vec<String>>,
   cdp_neighbors_detailed: Option<Vec<String>>,
   lldp_neighbors_simple :Option<Vec<String>>,
   lldp_neigbors_detailed: Option<Vec<String>>,
   license:Option<String>,
   ip_route_table_ipv4:Option<Vec<String>>,
   ip_route_table_ipv6:Option<Vec<String>>,


   // Important to parse in ::new()
   vendor: String,
   output:String

}

impl Utils{

    fn new(output:String,vendor:String) -> Self{
      Utils { interfaces_ipv4:None , interfaces_ipv6:None , current_routing_protocol:None, cdp_neighbors_simple:None, cdp_neighbors_detailed: None, lldp_neighbors_simple: None, lldp_neigbors_detailed:None,license:None, ip_route_table_ipv4:None, ip_route_table_ipv6:None,vendor,output}
    }
   // Regex for ipv4
    fn interfaces_ipv4(&mut self){
      if self.interfaces_ipv4.is_none(){
         if self.vendor.to_lowercase() == "cisco"{
            let output  = &self.output;
            let regex_pattern  = r"(?m)^(\S+)\s+\S+\s+\S+\s+\S+\s+up\s+up\s*$";
            let re  = Regex::new(regex_pattern).unwrap();
            let mut new_interface_array:Vec<String> = vec![];
            self.interfaces_ipv4 = Some((new_interface_array).clone());
            for caps in re.captures_iter(output){
               let the_thing_we_looking_for = caps[1].to_string();
               new_interface_array.push(the_thing_we_looking_for);


   

            }

         }

      }
    }

   // Regex for ipv6
   fn interfaces_ipv6(&mut self){}

   //Regex for current routing protocl
    fn current_routing_protocol(&mut self){}

   //Regex for cdp neighbors
    fn cdp_neighbors_simple(&mut self){}


   // Regex for cdp neighbors detailed 
    fn cdp_neighbors_detailed(&mut self ){}

   // Regex for lldp neighbors simple
    fn lldp_neighbors_simple(&mut self){}

   // Regex for lldp neighbors detailed 
    fn lldp_neigbors_detailed(&mut self){}


   





}



// This will match basically behavior so if it is Router ,Switch , Etc.. instead of doing each single class 
// This will basically be a choice between Router, Switch , Server ..
enum LaterBehavior{
   Router,
   Switch,
   Server
}

fn  choice(choice:LaterBehavior){
   match choice{
      LaterBehavior::Router => println!("Do the Router here"),
      LaterBehavior::Switch => println!("Do the Switch here"),
      LaterBehavior::Server => println!("Do the Server here")
   }
}




// Sending command via ssh2
struct CommandSSH <T:ToString>{
   //command:String, // Use && in the command if you want to make double 
   host_ip:T,
   host_name:T,
   host_password:T,
   output_given_after:Option<String>




}



impl <T:ToString> CommandSSH<T> {
   pub fn new (host_ip:T,host_name:T,host_password:T) -> Self{
      CommandSSH {host_ip,host_name,host_password,output_given_after:None}

   }
   pub fn process_command(&mut self,command:String){
      if self.output_given_after.is_none(){
            // Setting up Tcp Stream  and Session 
            let tcp:TcpStream = TcpStream::connect(self.host_ip.to_string()).unwrap();
            let mut session = Session::new().unwrap();
            session.set_tcp_stream(tcp);;
            session.handshake().unwrap();
            let username = &self.host_name;//.as_str();
            let password= &self.host_password;//.as_str();
            session.userauth_password(&username.to_string(),&password.to_string());  
            // This actually a shell
            let mut channel = session.channel_session().unwrap();
            // String to turn the output in 
            let mut string_use = String::new();
            // Executing the command via shell
            channel.exec(&command).unwrap();
            // Reading the command output 
            channel.read_to_string(&mut string_use).unwrap();

            // Updating the output_given to use Lazy Loading 
            self.output_given_after = Some((string_use));

            // Closing the channel 
            channel.wait_close();
      }    

         


   }
}





//Main which will prob be deleted this just for testing 
fn main()// ->  Result<(),ParseIntError>  
  {   let pattern = r"(?m)^(\S+)\s+\S+\s+\S+\s+\S+\s+up\s+up\s*$";
   let re = Regex::new(pattern).unwrap();
   let mut interfaces: Vec<String> = vec![];

   let data = r#"Interface              IP-Address      OK? Method Status                Protocol
FastEthernet0/0        192.168.1.1     YES manual up                    up
GigabitEthernet0/1     unassigned      YES unset  administratively down down
Loopback0              10.1.1.1        YES manual up                    up"#;
   for caps in re.captures_iter(data) {
      interfaces.push(caps[1].to_string());


   }
   let g = ">>";



   //File handling testing 

}

// What the router will have 
struct Router{
   host_ip: String,
   host_password:String,
   host_username:String,
   host_vendor:String,
   test   : Option<String>, // Basically Option is for Like None or a value , if value we use Some()
   interfaces_ipv4 : Option<Vec<String>>,
   hostname : Option<String>,

}










impl Router {
   pub  fn new(host_username:String,host_ip:String,host_password:String,host_vendor:String) -> Self{

      let vendor_list : [String;2] = ["cisco".to_string(),"juniper".to_string()];
      let lower_case : String = host_vendor.to_lowercase();
      if vendor_list.contains(&lower_case){
         let ping = Command::new("ping").arg("-c").arg("1").arg(&host_ip).output().expect("Failed to run the ping");
         if ping.status.success(){
            return Router{host_ip,host_password,host_vendor,host_username,test:None,interfaces_ipv4:None,hostname:None}
         }
         else {
             panic!("The Host is down try another one")
         }

         //return Router{host_ip,host_password,host_vendor,test:None,interfaces:None,hostname:None}
      }
      else {
          panic!("Use another vendor");
      }

      //Router{host_ip,host_password,host_vendor,test:None,interfaces:None,hostname:None}

}
   pub fn interfaces(&mut self ) {

      if self.interfaces_ipv4.is_none(){
         if self.host_vendor.to_lowercase() == "cisco"{
            let command = "enable && show ip interface brief".to_string();
             //Now use my SSH Struct to clearify code and send the command 
            let mut  new_command = CommandSSH::new(&self.host_ip, &self.host_username,&self.host_password);
            new_command.process_command(command);
            // The output we are going to parse after 
            let output = new_command.output_given_after.unwrap();
            // Using the Processing Class
            let let_made_g = Utils::new(output,"cisco".to_string());
            





      }
}

}






















}

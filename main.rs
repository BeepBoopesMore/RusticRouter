use std::num::{NonZero, ParseIntError};
use std::ptr::read;
use std::{cell::RefCell, ffi::FromVecWithNulError, os::unix::net::UnixDatagram};
use regex::{Captures, Regex};
use serde::de::value::Error;
use std::fs;
use std::io::{self, Read};
use std::collections::HashMap;
use ssh2::Session;
use std::net::TcpStream;
use std::process::Command;



// This will match basically behavior so if it is Router ,Switch , Etc.. instead of doing each single class 
// This will basically be a choice between Router, Switch , Server ..
enum LaterBehavior{
   Router,
   Switch,
   Server
}
struct CommandSSH <T:ToString>{
   command:String, // Use && in the command if you want to make double 
   host_ip:T,
   host_name:T,
   host_password:T,
   output_given_after:Option<String>


}



impl <T:ToString> CommandSSH<T> {
   pub fn new (command:String,host_ip:T,host_name:T,host_password:T) -> Self{
      CommandSSH {command,host_ip,host_name,host_password,output_given_after:None}

   }
   pub fn process_command(&mut self){
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
            channel.exec(&self.command).unwrap();
            // Reading the command output 
            channel.read_to_string(&mut string_use).unwrap();
           

            // Updating the output_given to use Lazy Loading 
            self.output_given_after = Some((string_use));


            // Closing the channel 
            channel.wait_close();
      }    

         


   }
}

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
   interfaces : Option<Vec<String>>,
   hostname : Option<String>,

}

impl Router {
   pub  fn new(host_username:String,host_ip:String,host_password:String,host_vendor:String) -> Self{

      let vendor_list : [String;2] = ["cisco".to_string(),"juniper".to_string()];
      let lower_case : String = host_vendor.to_lowercase();
      if vendor_list.contains(&lower_case){
         let ping = Command::new("ping").arg("-c").arg("1").arg(&host_ip).output().expect("Failed to run the ping");
         if ping.status.success(){
            return Router{host_ip,host_password,host_vendor,host_username,test:None,interfaces:None,hostname:None}
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
      if self.interfaces.is_none(){
         if self.host_vendor.to_lowercase() == "cisco"{
            let command = "enable && show ip interface brief".to_string();
            // Now use my SSH Struct to clearify code and send the command 
           let mut  comm: CommandSSH<&String> = CommandSSH::new(command,&self.host_ip,&self.host_username, &self.host_password);
           comm.process_command();



      }
}

}


      
   



































}

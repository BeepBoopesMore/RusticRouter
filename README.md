# RusticRouter
Just my frame-work imported for Rust UI for now still heavily working on it so people don't complain about no low level programming



# Things i want done 
- Making my raw arp packet sender for scanning
-Ui

#Code sample 



```rust 


use rTraffic::Router;

fn main(){

   let router1 = Router::new("host_username".to_string(),"host_ip".to_string(),"host_password".to_string(),"cisco|Cisco".to_string())
   router1.interfaces_ipv4();
   let interfaces = router1.interfaces_ipv4.unwrap();

}








```

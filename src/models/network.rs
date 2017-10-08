
/// A Struct to construct a network address
#[allow(dead_code)]
pub struct Network {
    pub is_ssl : bool,
    pub ip_address: String,
    pub port: u32
}

#[allow(dead_code)]
impl Network {
    pub fn get_url(&mut self) -> String {
        let url : String =
        if self.is_ssl == true {
             ["https://".to_string(), self.ip_address.to_string(), ":".to_string(), self.port.to_string(), "/".to_string()].join("").to_string()
        } else {
             ["http://".to_string(), self.ip_address.to_string(), ":".to_string(), self.port.to_string(), "/".to_string()].join("").to_string()
        };
        return url
    }
}

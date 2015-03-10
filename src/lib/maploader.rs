
use std::io::prelude::*;
use std::fs::File;
use serialize::json;

#[derive(Decodable,Encodable)]
#[derive(Debug)]
pub struct Cell { 
    pub id     : usize, 
    pub north  :usize, 
    pub south  :usize, 
    pub east   :usize, 
    pub west   :usize,
}

#[derive(Decodable,Encodable)]
#[derive(Debug)]
pub struct Map { 
    pub map_name : String, 
    pub count : usize, 
    pub cells : Vec<Cell> 
}
impl Map {
    #[allow(dead_code)] 
    pub fn load_map(map_name : &str) -> Map
    {
        let path = Path::new(map_name);
        let display = path.display();

        // Open the path in read-only mode, returns `IoResult<File>`
        let mut file = match File::open(&path) {
            // The `desc` field of `IoError` is a string that describes the error
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `IoResult<String>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            Ok(_) => print!("{} contains:\n{}", display, s),
        }

        let map : Map = match json::decode(&s[..]) {
            Err(why) => panic!("couldn't parse {}: {}", display, why),
            Ok(ok) => ok,
        };

        if map.count != map.cells.len() {
            panic!("expected {} cells, but was found {}!", map.count, map.cells.len());
        }

        return map;
    }
}
    

#[cfg(test)]
mod tests {
    use serialize::json;
    #[derive(Decodable,Encodable)]
    pub struct TestStruct { id : usize, map_name : String }
    #[test]
    pub fn encode_decode_json()
    {

        let value = TestStruct{ id : 9, map_name : "Demo".to_string(), };
        let js = json::encode(&value).unwrap();

        let loaded : TestStruct = json::decode(&js[..]).unwrap();
        assert!(value.id == loaded.id);
        assert!(value.map_name == loaded.map_name);
    }
}
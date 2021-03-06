
use std::io::prelude::*;
use std::io::Error;
use std::fs::File;
use std::path::Path;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Cell { 
    pub id     : usize, 
    pub north  : usize, 
    pub south  : usize, 
    pub east   : usize, 
    pub west   : usize,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Map { 
    pub name : String, 
    pub cells : Vec<Cell>, 
    pub start  : usize,
    pub target : usize,
}

impl Map {
    #[allow(dead_code)] 
    pub fn load(name : &str) -> Result<Map, Error> {
        let path = Path::new(name);
        let mut file = try!(File::open(&path));
        let mut s = String::new();
        try!(file.read_to_string(&mut s));
        let map : Map = json::decode(&s[..]).ok().expect("filed to decode json"); 
        return Ok(map);
    }

   #[allow(dead_code)] 
    pub fn save (name : &str, map : &Map) -> Result<(), Error> {
        let path = Path::new(name);
        let mut file = try!(File::create(&path));
        let js = json::encode(&map).ok().expect("failed to encode map");
        try!(file.write(&js.as_bytes()));
        Ok(())
    }

    pub fn demo_map() -> Map {
        Map{ 
            name : "Test Map".to_string(), 
            cells : vec![
                    Cell{ id:1, north:0, south:4, east:2, west:0 },
                    Cell{ id:2, north:0, south:5, east:3, west:1 },
                    Cell{ id:3, north:0, south:6, east:0, west:2 },
                    Cell{ id:4, north:1, south:7, east:5, west:0 },
                    Cell{ id:5, north:0, south:0, east:6, west:4 },
                    Cell{ id:6, north:3, south:9, east:0, west:5 },
                    Cell{ id:7, north:4, south:0, east:8, west:0 },
                    Cell{ id:8, north:0, south:0, east:9, west:7 },
                    Cell{ id:9, north:6, south:0, east:0, west:8 }
                ], 
            start : 1,
            target : 9,
        }
    }
}


#[cfg(test)]
mod tests {
    use rustc_serialize::json;
    #[derive(RustcDecodable, RustcEncodable, Debug)]
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

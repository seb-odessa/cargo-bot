
use std::io::prelude::*;
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
    pub map_name : String, 
    pub count : usize, 
    pub cells : Vec<Cell>, 
    pub begin_cell : usize,
    pub target_cell : usize,
}
impl Map {
    #[allow(dead_code)] 
    pub fn load_map(map_name : &str) -> Map {
        let path = Path::new(map_name);
        let display = path.display();

        // Open the path in read-only mode, returns `IoResult<File>`
        let mut file = match File::open(&path) {
            // The `desc` field of `IoError` is a string that describes the error
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `IoResult<String>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => println!("{} contains:\n{}", display, s),
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

    pub fn demo_map() -> Map {
        Map{ 
            map_name : "Test Map".to_string(), 
            count : 9, 
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
            begin_cell : 1,
            target_cell : 9,
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
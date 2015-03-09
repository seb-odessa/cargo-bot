use serialize;

#[derive(Decodable,Encodable)]
struct Cell { id : usize, north  :usize, south  :usize, east  :usize, west  :usize }

#[derive(Decodable,Encodable)]
struct Map { map_name : String, count : usize, cells : Vec<Cell> }


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
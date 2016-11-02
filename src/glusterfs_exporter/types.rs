extern crate serde;
extern crate serde_xml;

include!(concat!(env!("OUT_DIR"), "/glusterfs_exporter/types.rs"));

mod tests {
    use super::*;
    use serde_xml::de::from_iter;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn it_can_parse() -> () {
        let test_file = File::open("test_files/cumulative.xml").unwrap();
        let deserialized: Result<CliOutput,_> = from_iter(test_file.bytes());
        println!("{:?}", deserialized);
        assert!(deserialized.is_ok())
    }
}

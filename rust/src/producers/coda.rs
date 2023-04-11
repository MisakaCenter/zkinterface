use crate::ConstraintSystem;
use num_bigint::BigUint;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

macro_rules! bits_to_bytes {
    ($val:expr) => {
        ($val + 7) / 8
    };
}

/// serialize `BigUint` as a vector of u8 (in Little-endian), of a given size, padded with 0's if required.
fn serialize_biguint(biguint: BigUint, byte_len: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    let mut binary = biguint.to_bytes_le();
    let len = binary.len();
    ret.append(&mut binary);
    ret.append(&mut vec![0; byte_len - len]);

    ret
}

// parse coda json file and generate a constraint system
pub fn generate_from_coda<P: AsRef<Path>>(json_path: P) -> ConstraintSystem {
    // println!("json_path: {:?}", json_path.as_ref());
    let size_in_bytes = bits_to_bytes!(BigUint::from_str(
        "21888242871839275222246405745257275088548364400416034343698204186575808495617"
    )
    .unwrap()
    .bits()) as usize;
    let mut buf = Vec::<u8>::new();
    let mut file = std::fs::File::open(json_path).unwrap();
    file.read_to_end(&mut buf).unwrap();
    let json = serde_json::from_slice::<serde_json::Value>(&buf).unwrap();
    let mut constraints_vec: Vec<(
        (Vec<u64>, Vec<u8>),
        (Vec<u64>, Vec<u8>),
        (Vec<u64>, Vec<u8>),
    )> = Vec::new();

    let value = &json["constraints"];
    for (name, value) in value.as_array().unwrap().iter().enumerate() {
        let mut a_ids: Vec<u64> = Vec::new();
        let mut a_values: Vec<u8> = Vec::new();
        let mut b_ids: Vec<u64> = Vec::new();
        let mut b_values: Vec<u8> = Vec::new();
        let mut c_ids: Vec<u64> = Vec::new();
        let mut c_values: Vec<u8> = Vec::new();

        for item in value["a"].as_array().unwrap().iter() {
            let var = item["var"]
                .to_string()
                .replace("\"", "")
                .parse::<u64>()
                .unwrap();
            let value = BigUint::from_str(&item["value"].to_string().replace("\"", "")).unwrap();
            a_ids.push(var);
            a_values.append(&mut serialize_biguint(value, size_in_bytes));
        }

        for item in value["b"].as_array().unwrap().iter() {
            let var = item["var"]
                .to_string()
                .replace("\"", "")
                .parse::<u64>()
                .unwrap();
            let value = BigUint::from_str(&item["value"].to_string().replace("\"", "")).unwrap();
            b_ids.push(var);
            b_values.append(&mut serialize_biguint(value, size_in_bytes));
        }

        for item in value["c"].as_array().unwrap().iter() {
            let var = item["var"]
                .to_string()
                .replace("\"", "")
                .parse::<u64>()
                .unwrap();
            let value = BigUint::from_str(&item["value"].to_string().replace("\"", "")).unwrap();
            c_ids.push(var);
            c_values.append(&mut serialize_biguint(value, size_in_bytes));
        }

        constraints_vec.push(((a_ids, a_values), (b_ids, b_values), (c_ids, c_values)));
    }
    ConstraintSystem::from(constraints_vec.as_slice())
}

// {
//     "constraints": [{
//         "a": [{
//             "var": "3",
//             "value": "21888242871839275222246405745257275088548364400416034343698204186575808495616"
//         }],
//         "b": [{
//             "var": "2",
//             "value": "1"
//         }],
//         "c": [{
//             "var": "1",
//             "value": "21888242871839275222246405745257275088548364400416034343698204186575808495616"
//         }]
//     }]
// }

#![no_main]
use libfuzzer_sys::fuzz_target;
use bit_vec::{BitVec};

fuzz_target!(|data: &[u8]| {
    if data.len() > 3 {
        let opt = data[0];
        let half = (data.len() - 1) / 2;
        let end;
        if data.len() % 2 == 0 {
            end = 1;
        }
        else {
            end = 0;
        }
        let mut bv1 = BitVec::from_bytes(&data[1..half+1]);
        let mut bv2 = BitVec::from_bytes(&data[half+1..data.len()-end]);
        match opt {
            0=>{
                bv1.xor(&bv2);
                bv2.xnor(&bv1);
            },
            1=>{
                bv1.or(&bv2);
                bv2.nor(&bv1);
            },
            2=>{
                bv1.and(&bv2);
                bv2.nand(&bv1);
            }
            3=>{
                bv1.difference(&bv2);
                bv2.difference(&bv1);
            }
            4=>{
                bv1.append(&mut bv2);
                bv2.append(&mut bv1);
            }
            5=>{
                let _ = bv1 == bv2;
                _ = bv1 < bv2;
                _ = bv2 > bv2;
            }
            6=>{
                while bv2.len() > 0 {
                    bv1.push(bv2.pop().expect("FAIL"));
                }
                while bv1.len() > 0 {
                    bv2.push(bv1.pop().expect("FAIL"));
                }
            }
            7=>{
                let _ = bv1.all();
                _ = bv1.any();
                _ = bv1.none();
            }
            8=>{
                bv1.extend(bv2.iter());
                bv2.extend(bv1.iter());
            }
            9=>{
                let _bv3 = bv1.split_off(bv1.len() / 2);
            }
            10=>{
                bv1.append(&mut bv2);
                let serialized = serde_json::to_string(&bv1).unwrap();
                let _unserialized: BitVec = serde_json::from_str(&serialized).unwrap();
            }
            _=> (),
        }
    }
});

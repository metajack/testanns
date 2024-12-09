use std::io::{self, BufReader, BufWriter, ErrorKind, Write};
use std::fs::File;
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub fn load_fvecs<P: AsRef<Path>>(path: P) -> io::Result<Vec<Vec<f32>>> {
    let f = File::open(path)?;
    let mut r = BufReader::new(f);

    let mut vs = vec![];

    loop {
        let n = r.read_i32::<LittleEndian>();
        match n {
            Ok(n) => {
                let n = n as usize;
                assert!(n <= 1024);

                let mut v = Vec::with_capacity(n);
                
                for _ in 0..n {
                    v.push(r.read_f32::<LittleEndian>()?);
                }
        
                vs.push(v);
            },
            Err(e) => {
                if e.kind() == ErrorKind::UnexpectedEof {
                    return Ok(vs);
                } else {
                    return Err(e);
                }
            },
        };
    }
}

pub fn save_fvecs<P: AsRef<Path>>(path: P, vs: &Vec<Vec<f32>>) -> io::Result<()> {
    assert!(!vs.is_empty());

    let f = File::create(path)?;
    let mut w = BufWriter::new(f);

    for v in vs {
        w.write_i32::<LittleEndian>(v.len() as i32)?;
        for &e in v {
            w.write_f32::<LittleEndian>(e)?;
        }
    }
    w.flush()?;

    Ok(())
}
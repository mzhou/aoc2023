use std::array::from_fn;
use std::error::Error;
use std::io::stdin;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Lense {
    focal_length: u8,
    label: Vec<u8>,
}

fn main() -> Result<(), Box<dyn Error>> {
    for line in stdin().lines() {
        let line = line?;

        let mut boxes: [Vec<Lense>; 256] = from_fn(|_| Vec::<Lense>::new());

        for section in line.as_bytes().split(|b| *b == b',') {
            let mut hash = 0usize;
            let mut label = Vec::<u8>::new();
            let mut found_equal = false;
            for b in section.iter().cloned() {
                if b == b'-' {
                    let the_box = &mut boxes[hash];
                    if let Some(pos) = the_box.iter().position(|lense| lense.label == label) {
                        the_box.remove(pos);
                    }
                    break;
                }
                if b == b'=' {
                    found_equal = true;
                    continue;
                }
                if found_equal {
                    let the_box = &mut boxes[hash];
                    let focal_length = b - b'0';
                    if let Some(lense) = the_box.iter_mut().find(|lense| lense.label == label) {
                        lense.focal_length = focal_length;
                    } else {
                        let lense = Lense {
                            focal_length,
                            label,
                        };
                        the_box.push(lense);
                    }
                    break;
                }
                label.push(b);
                hash += b as usize;
                hash *= 17;
                hash = hash & 0xff;
            }
        }

        let mut total_power = 0usize;

        for (box_num, the_box) in boxes.iter().enumerate() {
            for (slot_num, lense) in the_box.iter().enumerate() {
                let power = (box_num + 1) * (slot_num + 1) * (lense.focal_length as usize);
                total_power += power;
            }
        }

        eprintln!("{}", total_power);
    }

    Ok(())
}

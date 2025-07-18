use std::collections::{HashMap, HashSet};
use std::io::{self, Cursor, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct Model {
    tk_output: HashMap<u16, Vec<i32>>, // Transition table
    nb_numfeats: usize,
    tk_nextmove: Vec<u16>,
    norm_probs: bool,
    data: ModelData,
    used_data: Option<ModelData>,
}

#[derive(Debug)]
pub struct ModelData {
    pub nb_classes: Vec<String>, // List of class names
    pub nb_ptc: Vec<Vec<f32>>,   // 2D array
    pub nb_pc: Vec<f32>,         // 1D array
}

pub enum Error {
    UnknownLanguageCode(String),
    NoLanguage,
}

impl Model {
    fn data(&self) -> &ModelData {
        if let Some(data) = &self.used_data {
            data
        } else {
            &self.data
        }
    }
}

impl Model {
    pub fn set_langs(&mut self, langs: Option<HashSet<String>>) -> Result<(), Error> {
        if let Some(langs) = langs {
            if langs.len() < 2 {
                return Err(Error::NoLanguage);
            }
            let unknown = langs
                .iter()
                .find(|lang| !self.data.nb_classes.contains(&lang));
            if let Some(lang) = unknown {
                return Err(Error::UnknownLanguageCode(lang.to_owned()));
            }
            let subset_mask = self
                .data
                .nb_classes
                .iter()
                .map(|s| langs.contains(s))
                .collect::<Vec<_>>();
            let nb_classes = self
                .data
                .nb_classes
                .iter()
                .filter(|s| langs.contains(*s))
                .cloned()
                .collect::<Vec<_>>();
            let nb_pc = self
                .data
                .nb_pc
                .iter()
                .zip(&subset_mask)
                .filter(|(_, m)| **m)
                .map(|v| *v.0)
                .collect::<Vec<_>>();

            let nb_ptc = self
                .data
                .nb_ptc
                .iter()
                .map(|v| {
                    v.iter()
                        .zip(&subset_mask)
                        .filter(|(_, m)| **m)
                        .map(|v| *v.0)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            self.used_data = Some(ModelData {
                nb_classes,
                nb_ptc,
                nb_pc,
            });
        } else {
            self.used_data = None;
        }
        Ok(())
    }

    fn apply_norm_probs(&self, pd: Vec<f32>) -> Vec<f32> {
        if self.norm_probs {
            todo!("pd = 1/np.exp(pd[None,:] - pd[:,None]).sum(1)")
        } else {
            pd
        }
    }

    /// Return a list of languages in order of likelihood.
    pub fn rank(&self, text: &str) -> Vec<(&str, f32)> {
        let fv: Vec<u16> = self.instance2fv(text);
        let probs: Vec<f32> = self.apply_norm_probs(self.nb_classprobs(fv));
        let mut class_probs: Vec<(&str, f32)> = self
            .data()
            .nb_classes
            .iter()
            .map(|class| class.as_str())
            .zip(probs.into_iter())
            .collect();

        class_probs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        class_probs
    }

    /// Classify an instance.
    /// Will return None if no language is detected.
    pub fn classify(&self, text: &str) -> Option<(&str, f32)> {
        let fv = self.instance2fv(text);
        let probs = self.apply_norm_probs(self.nb_classprobs(fv));
        let (max_index, max_prob) = probs
            .iter()
            .enumerate()
            .fold(None, |max, (idx, &prob)| match max {
                None => Some((idx, prob)),
                Some((max_idx, max_val)) => {
                    if prob > max_val {
                        Some((idx, prob))
                    } else {
                        Some((max_idx, max_val))
                    }
                }
            })
            .unzip();
        let (max_index, max_prob) = (max_index?, max_prob?);

        self.data()
            .nb_classes
            .get(max_index)
            .map(|class| (class.as_str(), max_prob))
    }

    fn nb_classprobs(&self, fv: Vec<u16>) -> Vec<f32> {
        // let pdc = np.dot(fv, self.data.nb_ptc);
        // pdc + self.data.nb_pc

        // dot
        let m = fv.len();
        let n = self.data().nb_pc.len();

        let mut pdc = vec![0f32; n];

        for i in 0..m {
            let fv_val = fv[i] as f32;
            for j in 0..n {
                pdc[j] += fv_val * self.data().nb_ptc[i][j];
            }
        }

        // add
        for j in 0..n {
            pdc[j] += self.data().nb_pc[j];
        }

        pdc
    }

    fn instance2fv(&self, text: &str) -> Vec<u16> {
        let indexes = text
            .chars()
            .fold((0usize, Vec::new()), |(state, mut acc), letter| {
                let new_state = self.tk_nextmove[(state << 8) + letter as usize];
                let new_state_u = new_state as usize;
                let output = self.tk_output.get(&new_state).cloned().unwrap_or_default();
                acc.extend(output);
                (new_state_u, acc)
            })
            .1;
        let mut arr = vec![0; self.nb_numfeats];

        let counts = counter(&indexes);

        for (index, value) in counts {
            arr[index as usize] = value;
        }
        arr
    }
}

fn counter(indexes: &Vec<i32>) -> HashMap<i32, u16> {
    let mut counts = HashMap::new();

    for inner_vec in indexes {
        *counts.entry(*inner_vec).or_insert(0) += 1;
    }

    counts
}
fn read_u32(reader: &mut impl Read) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_f32_vec(reader: &mut impl Read, len: usize) -> io::Result<Vec<f32>> {
    let mut buf = vec![0u8; len * 4];
    reader.read_exact(&mut buf)?;
    let floats = buf
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    Ok(floats)
}

fn read_u16_vec(reader: &mut impl Read, len: usize) -> io::Result<Vec<u16>> {
    let mut buf = vec![0u8; len * 2];
    reader.read_exact(&mut buf)?;
    let floats = buf
        .chunks_exact(2)
        .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
        .collect();
    Ok(floats)
}

fn read_string(reader: &mut impl Read) -> io::Result<String> {
    let len = read_u32(reader)? as usize;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf).expect("Invalid UTF-8"))
}

fn read_i32_vec(reader: &mut impl Read, len: usize) -> io::Result<Vec<i32>> {
    let mut buf = vec![0u8; len * 4];
    reader.read_exact(&mut buf)?;
    let vals = buf
        .chunks_exact(4)
        .map(|b| i32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    Ok(vals)
}

impl Model {
    pub fn load() -> io::Result<Self> {
        let mut reader = Cursor::new(include_bytes!("model.bin"));

        let rows = read_u32(&mut reader)? as usize;
        let cols = read_u32(&mut reader)? as usize;
        let nb_ptc_flat = read_f32_vec(&mut reader, rows * cols)?;
        let nb_ptc: Vec<Vec<f32>> = nb_ptc_flat
            .chunks_exact(cols)
            .map(|row| row.to_vec())
            .collect();

        let nb_pc_len = read_u32(&mut reader)? as usize;
        let nb_pc = read_f32_vec(&mut reader, nb_pc_len)?;

        let tk_nextmove_len = read_u32(&mut reader)? as usize;
        let tk_nextmove = read_u16_vec(&mut reader, tk_nextmove_len)?;

        let nb_class_count = read_u32(&mut reader)? as usize;
        let mut nb_classes = Vec::with_capacity(nb_class_count);
        for _ in 0..nb_class_count {
            nb_classes.push(read_string(&mut reader)?);
        }

        let tk_output_count = read_u32(&mut reader)? as usize;
        let mut tk_output = HashMap::with_capacity(tk_output_count);
        for _ in 0..tk_output_count {
            let key = read_u32(&mut reader)?;
            let key = match u16::try_from(key) {
                Ok(v) => v,
                Err(_) => unreachable!("Key does not fit in u16"),
            };
            let val_len = read_u32(&mut reader)? as usize;
            let val = read_i32_vec(&mut reader, val_len)?;
            tk_output.insert(key, val);
        }

        let nb_numfeats = nb_ptc.iter().map(|v| v.len()).sum::<usize>() / nb_pc.len();
        assert_eq!(bytes_remaining(&mut reader)?, 0);
        Ok(Self {
            norm_probs: false,
            used_data: None,
            nb_numfeats,
            data: ModelData {
                nb_classes,
                nb_ptc,
                nb_pc,
            },
            tk_nextmove,
            tk_output,
        })
    }
}

fn bytes_remaining<R: Read + Seek>(reader: &mut R) -> std::io::Result<u64> {
    let current = reader.seek(SeekFrom::Current(0))?;
    let end = reader.seek(SeekFrom::End(0))?;
    reader.seek(SeekFrom::Start(current))?; // Restore position
    Ok(end - current)
}

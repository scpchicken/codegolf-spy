use serde_json;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::string::String;
use tabled::{Table, Tabled};

#[derive(Debug, Default, Clone)]
struct Stat {
  byte_min: isize,
  byte_min_hash: HashMap<String, (isize, isize)>,
}

#[derive(Tabled)]
struct Sol {
  hole: String,
  lang: String,
  point: isize,
}

fn main() {
  let data = fs::read_to_string("../bruh.json").expect("Unable to read json file");

  let json: serde_json::Value =
    serde_json::from_str(&data).expect("JSON does not have correct format.");

  let mut hash: HashMap<String, Stat> = HashMap::new();
  let login_name = std::env::args().nth(1).unwrap();
  let mut login_hash: HashMap<String, Stat> = HashMap::new();
  let mut dup_hash: HashMap<String, usize> = HashMap::new();
  let mut point_total = 0;

  for x in json.as_array().unwrap().into_iter() {
    let scoring = x["scoring"].as_str().unwrap().to_string();

    match scoring.as_str() {
      "chars" => continue,
      "bytes" => {}
      _ => panic!("invalid scoring"),
    };

    let lang = x["lang"].as_str().unwrap().to_string();
    let hole = x["hole"].as_str().unwrap().to_string();
    let login = x["login"].as_str().unwrap().to_string();
    let byte = x["bytes"].as_i64().unwrap() as isize;
    let mut stat = hash.entry(hole.clone()).or_insert(Stat {
      byte_min: 69696969420,
      byte_min_hash: HashMap::new(),
    });

    let login_stat = login_hash.entry(hole.clone()).or_insert(Stat::default());
    match login_stat.clone().byte_min_hash.clone().get(&lang) {
      Some(_) => {}
      None => {
        login_stat
          .byte_min_hash
          .insert(lang.clone(), (696969420, 696969420));
      }
    };

    if login == login_name {
      let login_stat = login_hash.entry(hole.clone()).or_insert(Stat::default());
      match login_stat.clone().byte_min_hash.clone().get(&lang) {
        Some((m, _)) => {
          if byte < *m {
            login_stat
              .byte_min_hash
              .insert(lang.clone(), (byte, 696969420));
          }
        }
        None => panic!("should have init login before"),
      }
    }

    if byte < stat.byte_min {
      stat.byte_min = byte;
    }

    stat
      .byte_min_hash
      .entry(lang.clone())
      .or_insert((696969420, 0));
    match stat.byte_min_hash.clone().get(&lang) {
      Some((m, s)) => {
        let s_new = if dup_hash.contains_key(&format!("{} {} {}", hole, lang, login)) {
          s + 0
        } else {
          dup_hash.insert(format!("{} {} {}", hole, lang, login), 42069);
          s + 1
        };

        stat.byte_min_hash.insert(lang.clone(), (*m, s_new));
        if byte < *m {
          stat.byte_min_hash.insert(lang.clone(), (byte, s_new));
        }
      }
      None => panic!("should have init stat before"),
    }
  }

  let mut login_top = login_hash
    .into_iter()
    .map(|(hole, l)| {
      let login_byte_min_vec = l
        .clone()
        .byte_min_hash
        .clone()
        .into_iter()
        .collect::<Vec<_>>();
      let byte_min = hash[&hole].clone().byte_min;

      let mut login_byte_score_vec = login_byte_min_vec
        .into_iter()
        .map(|(login_lang, (login_byte, _))| {
          let h = hash[&hole].clone().byte_min_hash.clone();
          let (lang, (lang_byte_min, sol_count)) = h
            .clone()
            .into_iter()
            .find(|(lang, _)| lang == &login_lang)
            .unwrap();

          let sqrt_n = (sol_count as f64).sqrt();

          // Sb = ((√n + 2) ÷ (√n + 3)) × S + (1 ÷ (√n + 3)) × Sa
          // Points = Sb ÷ Su × 1000
          let sb = ((sqrt_n + 2.0) / (sqrt_n + 3.0)) * lang_byte_min as f64 +
            (1.0 / (sqrt_n + 3.0)) * byte_min as f64;
          let point = sb / login_byte as f64 * 1000.0;
          (hole.clone(), lang, point)
        })
        .collect::<Vec<_>>();

      login_byte_score_vec.sort_by(|a, b| match b.2.partial_cmp(&a.2).unwrap() {
        Ordering::Equal => a.1.cmp(&b.1),
        o => o,
      });

      let (hole, lang, point) = login_byte_score_vec.into_iter().next().unwrap();

      // rounding each time gives more accurate results than using f64 and rounding at the end
      point_total += point.round() as isize;
      Sol {
        hole,
        lang: if point == 0.0 {
          "N/A".to_string()
        } else {
          lang
        },
        point: point.round() as isize,
      }
    })
    .collect::<Vec<_>>();

  login_top.sort_unstable_by_key(|x| (-x.point, x.hole.clone()));
  println!(
    "{}\n\ntotal: {}",
    Table::new(login_top).to_string(),
    point_total
  );
}

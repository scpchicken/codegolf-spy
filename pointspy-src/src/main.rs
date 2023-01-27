#![allow(deprecated)]
use chrono::{Datelike, Duration, TimeZone, Utc};
use plotters::prelude::*;
use plotters::style::colors::full_palette;
use regex::Regex;
use serde_json;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::string::String;

#[derive(Debug, Default, Clone)]
struct Stat {
  byte_min: isize,
  byte_min_hash: HashMap<String, (isize, isize)>,
}

struct Sol {
  hole: String,
  lang: String,
  point: isize,
}

const OUT_FILE_NAME: &'static str = "../bruh.png";

fn main() -> Result<(), Box<dyn Error>> {
  let data = fs::read_to_string("../bruh.json").expect("Unable to read json file");

  let json: serde_json::Value =
    serde_json::from_str(&data).expect("JSON does not have correct format.");

  let mut hash: HashMap<String, Stat> = HashMap::new();
  let login_name_vec = std::env::args().skip(1).collect::<Vec<_>>();
  let mut login_hash: HashMap<String, HashMap<String, Stat>> = HashMap::new();
  let mut dup_hash: HashMap<String, usize> = HashMap::new();
  let date_re = Regex::new(r"^(\d+)-(\d+)-(\d+)T").unwrap();
  let mut date_join = None;

  let mut json_vec = json
    .as_array()
    .unwrap()
    .into_iter()
    .map(|x| {
      let date = x["submitted"].as_str().unwrap().to_string();
      let cap = date_re.captures(&date).unwrap();
      (
        x,
        Utc
          .with_ymd_and_hms(
            cap.get(1).unwrap().as_str().parse::<usize>().unwrap() as i32,
            cap.get(2).unwrap().as_str().parse::<usize>().unwrap() as u32,
            cap.get(3).unwrap().as_str().parse::<usize>().unwrap() as u32,
            0,
            0,
            0,
          )
          .unwrap(),
      )
    })
    .collect::<Vec<_>>();

  json_vec.sort_unstable_by_key(|x| (x.1));
  let mut json_ind = 0;
  let mut date_check = json_vec.clone().into_iter().next().unwrap().1 + Duration::days(1);
  let mut graph_point_hash = HashMap::new();

  while json_vec.clone().into_iter().nth(json_ind).is_some() {
    let mut point_total = 0;
    for (x, date_curr) in json_vec.clone().into_iter().skip(json_ind) {
      if date_curr >= date_check {
        date_check = date_check + Duration::days(1);
        break;
      }

      let lang = x["lang"].as_str().unwrap().to_string();
      let hole = x["hole"].as_str().unwrap().to_string();
      let login = x["login"].as_str().unwrap().to_string();
      let byte = x["bytes"].as_i64().unwrap() as isize;
      let mut stat = hash.entry(hole.clone()).or_insert(Stat {
        byte_min: 69696969420,
        byte_min_hash: HashMap::new(),
      });

      if login_name_vec.contains(&login) {
        let lsh = login_hash.entry(login.clone()).or_insert(HashMap::new());
        let login_stat = lsh.entry(hole.clone()).or_insert(Stat::default());
        match login_stat.clone().byte_min_hash.clone().get(&lang) {
          Some(_) => {}
          None => {
            login_stat
              .byte_min_hash
              .insert(lang.clone(), (696969420, 696969420));
          }
        };
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
      };

      json_ind += 1
    }

    for (login_name, lh) in login_hash.clone().into_iter() {
      let mut point_total = 0;
      let mut login_top = lh
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
      if point_total != 0 {
        let mut graph_point_vec = graph_point_hash
          .entry(login_name.clone())
          .or_insert(vec![(date_check - Duration::days(2), 0_f64)]);

        match date_join {
          Some(_) => {}
          None => {
            date_join = Some(date_check - Duration::days(2));
          }
        }
        graph_point_vec.push((date_check - Duration::days(1), point_total as f64));
      }
    }
  }

  let root = BitMapBackend::new(OUT_FILE_NAME, (1920, 1080)).into_drawing_area();
  root.fill(&RGBColor(50, 50, 50))?;
  let mut chart = ChartBuilder::on(&root)
    .margin(10)
    .caption(
      format!("total points on code.golf (bytes)").as_str(),
      ("sans-serif", 40, &WHITE),
    )
    .set_label_area_size(LabelAreaPosition::Left, 60)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .build_cartesian_2d(
      (date_join.unwrap().date()..(Utc::now() + Duration::days(1)).date()).monthly(),
      0.0..graph_point_hash.clone().into_iter().fold(0_f64, |acc, x| {
        f64::max(
          acc,
          x.1.into_iter().fold(0_f64, |bcc, y| f64::max(bcc, y.1)),
        )
      }),
    )?;

  let axis_style = ("sans-serif", 13).into_font().color(&WHITE);

  chart
    .configure_mesh()
    .disable_x_mesh()
    .disable_y_mesh()
    .x_labels(30)
    .y_labels(30)
    .x_label_style(axis_style.clone())
    .y_label_style(axis_style.clone())
    .max_light_lines(4)
    .draw()?;

  for (idx, login_name) in login_name_vec.clone().into_iter().enumerate() {
    let colour = Palette99::pick(idx).mix(0.9);
    chart
      .draw_series(LineSeries::new(
        match graph_point_hash.clone().get(&login_name) {
          Some(gpv) => gpv
            .iter()
            .map(|(d, point)| (Utc.ymd(d.year(), d.month(), d.day()), *point)),
          None => panic!("human/robot {} is unalive", login_name),
        },
        colour.stroke_width(1),
      ))?
      .label(login_name.clone())
      .legend(move |(x, y)| Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], colour.filled()));
  }

  chart
    .configure_series_labels()
    .position(SeriesLabelPosition::MiddleLeft)
    .background_style(&full_palette::BLUEGREY_800)
    .label_font(("sans-serif", 20, &WHITE))
    .border_style(&BLACK)
    .draw()?;

  println!("Result has been saved to {}", OUT_FILE_NAME);
  Ok(())
}

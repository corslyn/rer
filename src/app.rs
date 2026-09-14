use chrono::{DateTime, Utc};

use crate::siri::Root;

#[derive(Debug, Clone)]
pub struct Station {
    pub id: String,
    pub name: String,
}

pub fn load_stations() -> Result<Vec<Station>, csv::Error> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(include_bytes!("../zones-d-arrets.csv").as_slice());

    reader
        .deserialize::<StationRecord>()
        .map(|record| {
            record.map(|record| Station {
                id: record.id,
                name: record.name,
            })
        })
        .collect()
}

#[derive(Debug, serde::Deserialize)]
struct StationRecord {
    #[serde(rename = "zdaid")]
    id: String,
    #[serde(rename = "zdaname")]
    name: String,
}

#[derive(Debug)]
pub struct Departure {
    pub eta: i64,
    pub line: String,
    pub train: String,
    pub stop: String,
    pub destination: String,
}

// c super pas pretty lol
pub fn pretty_print(root: &Root) -> Result<(), Box<dyn std::error::Error>> {
    println!("Prochains passages");
    println!(
        "Réponse reçue à : {:?}",
        root.siri.service_delivery.response_timestamp
    );
    for delivery in &root.siri.service_delivery.stop_monitoring_delivery {
        println!("Réponse reçue à : {:?}", delivery.response_timestamp);
        for visit in &delivery.monitored_stop_visit {
            println!(
                "Passage à {:?} pour le véhicule {:?}, destination : {:?} (Ligne : {:?})",
                visit.recorded_at_time,
                visit
                    .monitored_vehicle_journey
                    .as_ref()
                    .map(crate::siri::get_train_name)
                    .unwrap_or(Some("Pas de nom de train")),
                visit
                    .monitored_vehicle_journey
                    .as_ref()
                    .map(crate::siri::get_destination_name)
                    .unwrap_or(Some("Pas de destination")),
                visit
                    .monitored_vehicle_journey
                    .as_ref()
                    .map(crate::siri::localized_line_name)
                    .unwrap_or(Some("Pas de ligne"))
            );
        }
    }
    Ok(())
}
pub struct App {
    pub station: String,
    pub input: String,
    pub line: Option<String>,
    pub departures: Vec<Departure>,
    pub status: String,
    api_key: String,
    station_id: String,
    stations: Vec<Station>,
    suggestion_index: Option<usize>,
}

impl App {
    pub fn new(
        station: &str,
        station_id: &str,
        api_key: String,
        root: Root,
        stations: Vec<Station>,
    ) -> Self {
        let mut app = Self {
            station: station.to_owned(),
            input: station.to_owned(),
            line: None,
            departures: Vec::new(),
            status: String::new(),
            api_key,
            station_id: station_id.to_owned(),
            stations,
            suggestion_index: None,
        };
        app.set_data(root);
        app
    }

    pub fn suggestions(&self) -> Vec<&Station> {
        let query = self.input.trim().to_lowercase();
        if query.is_empty() {
            return Vec::new();
        }

        self.stations
            .iter()
            .filter(|station| station.name.to_lowercase().contains(&query))
            .take(6)
            .collect()
    }

    pub fn suggestion_is_selected(&self, index: usize) -> bool {
        self.suggestion_index == Some(index)
    }

    pub fn input_char(&mut self, character: char) {
        self.input.push(character);
        self.suggestion_index = None;
    }

    pub fn input_backspace(&mut self) {
        self.input.pop();
        self.suggestion_index = None;
    }

    pub fn move_suggestion(&mut self, direction: i32) {
        let count = self.suggestions().len();
        if count == 0 {
            return;
        }

        let current = self.suggestion_index.unwrap_or(0) as i32;
        self.suggestion_index =
            Some((current + direction).rem_euclid(count as i32) as usize);
    }

    pub fn select_suggestion(&mut self) -> bool {
        let suggestions = self.suggestions();
        let Some(index) = self.suggestion_index else {
            return false;
        };
        let Some(station) = suggestions.get(index) else {
            return false;
        };
        let name = station.name.clone();
        let id = station.id.clone();

        self.input = name.clone();
        self.station = name;
        self.station_id = id;
        self.suggestion_index = None;
        true
    }

    pub fn refresh(&mut self) {
        let response =
            match crate::api::request(&self.station_id, &self.api_key) {
                Ok(response) => response,
                Err(error) => {
                    self.status = format!("CPT: {error}");
                    return;
                }
            };
        let root = match crate::api::parse_siri(&response) {
            Ok(root) => root,
            Err(error) => {
                self.status = format!("CPT: {error}");
                return;
            }
        };

        self.set_data(root);
        self.status = "Affichage mis a jour".to_owned();
    }

    fn set_data(&mut self, root: Root) {
        self.departures = root
            .siri
            .service_delivery
            .stop_monitoring_delivery
            .into_iter()
            .flat_map(|delivery| delivery.monitored_stop_visit)
            .filter_map(|visit| {
                let monitoring_ref = visit
                    .monitoring_ref
                    .as_ref()
                    .and_then(|reference| reference.value.as_deref())?;
                if !monitoring_ref.ends_with(&format!(":{}:", self.station_id))
                    && monitoring_ref != self.station_id
                {
                    return None;
                }
                let journey = visit.monitored_vehicle_journey?;
                let line = crate::siri::localized_line_name(&journey)
                    .unwrap_or("?")
                    .to_owned();
                let train = crate::siri::get_train_name(&journey)
                    .unwrap_or("pas de nom de train")
                    .to_owned();
                let destination = crate::siri::get_destination_name(&journey)
                    .unwrap_or("jsp ou ca va mdr")
                    .to_owned();
                let call = journey.monitored_call?;

                let eta = match call.expected_arrival_time {
                    Some(time) => time
                        .parse::<DateTime<Utc>>()
                        .expect("CPT")
                        .signed_duration_since(Utc::now())
                        .num_minutes(),
                    None => Utc::now()
                        .signed_duration_since(Utc::now())
                        .num_minutes(),
                };

                if eta.is_negative() {
                    return None;
                }

                Some(Departure {
                    eta,
                    line,
                    train,
                    stop: call
                        .stop_point_name
                        .first()
                        .and_then(|value| value.value.clone())
                        .unwrap_or_else(|| "jsp ca s'arrete ou".to_owned()),
                    destination,
                })
            })
            .collect();
    }
}

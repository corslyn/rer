use crate::siri::Root;

#[derive(Debug)]
pub struct Departure {
    pub eta: String,
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
}

impl App {
    pub fn new(station: &str, api_key: String, root: Root) -> Self {
        let mut app = Self {
            station: station.to_owned(),
            input: station.to_owned(),
            line: None,
            departures: Vec::new(),
            status: String::new(),
            api_key,
        };
        app.set_data(root);
        app
    }

    pub fn refresh(&mut self) {
        let response = match crate::api::request(&self.input, &self.api_key) {
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

        self.station = self.input.clone();
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
                if !monitoring_ref.ends_with(&format!(":{}:", self.station))
                    && monitoring_ref != self.station
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
                Some(Departure {
                    eta: call
                        .expected_arrival_time
                        .or(call.expected_departure_time)
                        .unwrap_or_else(|| "--".to_owned()),
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

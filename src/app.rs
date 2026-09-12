use crate::siri::Root;

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

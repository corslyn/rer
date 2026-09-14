use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Root {
    #[serde(rename = "Siri")]
    pub siri: Siri,
}

#[derive(Debug, Deserialize)]
pub struct Siri {
    #[serde(rename = "ServiceDelivery")]
    pub service_delivery: ServiceDelivery,
}

#[derive(Debug, Deserialize)]
pub struct ServiceDelivery {
    #[serde(rename = "ResponseTimestamp")]
    pub response_timestamp: Option<String>,

    #[serde(rename = "ProducerRef")]
    pub producer_ref: Option<String>,

    #[serde(rename = "StopMonitoringDelivery", default)]
    pub stop_monitoring_delivery: Vec<StopMonitoringDelivery>,
}

#[derive(Debug, Deserialize)]
pub struct StopMonitoringDelivery {
    #[serde(rename = "ResponseTimestamp")]
    pub response_timestamp: Option<String>,

    #[serde(rename = "Version")]
    pub version: Option<String>,

    #[serde(rename = "Status")]
    pub status: Option<String>,

    #[serde(rename = "MonitoredStopVisit", default)]
    pub monitored_stop_visit: Vec<MonitoredStopVisit>,
}

#[derive(Debug, Deserialize)]
pub struct MonitoredStopVisit {
    #[serde(rename = "RecordedAtTime")]
    pub recorded_at_time: Option<String>,

    #[serde(rename = "ItemIdentifier")]
    pub item_identifier: Option<String>,

    #[serde(rename = "MonitoringRef")]
    pub monitoring_ref: Option<RefValue>,

    #[serde(rename = "MonitoredVehicleJourney")]
    pub monitored_vehicle_journey: Option<MonitoredVehicleJourney>,
}

#[derive(Debug, Deserialize)]
pub struct RefValue {
    #[serde(rename = "value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MonitoredVehicleJourney {
    #[serde(rename = "LineRef")]
    pub line_ref: Option<RefValue>,

    #[serde(rename = "DestinationRef")]
    pub destination_ref: Option<RefValue>,

    #[serde(rename = "DestinationName", default)]
    pub destination_name: Vec<LocalizedValue>,

    #[serde(rename = "VehicleJourneyName", default)]
    pub vehicle_journey_name: Vec<LocalizedValue>,

    #[serde(rename = "JourneyNote", default)]
    pub journey_note: Vec<LocalizedValue>,

    #[serde(rename = "MonitoredCall")]
    pub monitored_call: Option<MonitoredCall>,

    #[serde(rename = "TrainNumbers")]
    pub train_numbers: Option<TrainNumbers>,

    #[serde(rename = "VehicleFeatureRef", default)]
    pub vehicle_feature_ref: Vec<String>,

    #[serde(rename = "DirectionRef")]
    pub direction_ref: Option<RefValue>,
}

#[derive(Debug, Deserialize)]
pub struct LocalizedValue {
    #[serde(rename = "value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MonitoredCall {
    #[serde(rename = "StopPointName", default)]
    pub stop_point_name: Vec<LocalizedValue>,

    #[serde(rename = "VehicleAtStop")]
    pub vehicle_at_stop: Option<bool>,

    #[serde(rename = "DestinationDisplay", default)]
    pub destination_display: Vec<LocalizedValue>,

    #[serde(rename = "ExpectedArrivalTime")]
    pub expected_arrival_time: Option<String>,

    #[serde(rename = "ExpectedDepartureTime")]
    pub expected_departure_time: Option<String>,

    #[serde(rename = "AimedArrivalTime")]
    pub aimed_arrival_time: Option<String>,

    #[serde(rename = "AimedDepartureTime")]
    pub aimed_departure_time: Option<String>,

    #[serde(rename = "DeparturePlatformName")]
    pub departure_platform_name: Option<LocalizedValue>,

    #[serde(rename = "ArrivalPlatformName")]
    pub arrival_platform_name: Option<LocalizedValue>,

    #[serde(rename = "DepartureStatus")]
    pub departure_status: Option<String>,

    #[serde(rename = "ArrivalStatus")]
    pub arrival_status: Option<String>,

    #[serde(rename = "Order")]
    pub order: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TrainNumbers {
    #[serde(rename = "TrainNumberRef", default)]
    pub train_number_ref: Vec<RefValue>,
}

pub fn get_platform_name(journey: &MonitoredVehicleJourney) -> Option<&str> {
    journey
        .monitored_call
        .as_ref()
        .and_then(|call| call.departure_platform_name.as_ref())
        .and_then(|platform| platform.value.as_deref())
}

pub fn get_train_name(journey: &MonitoredVehicleJourney) -> Option<&str> {
    journey
        .vehicle_journey_name
        .first()
        .and_then(|localized| localized.value.as_deref())
}

pub fn get_destination_name(journey: &MonitoredVehicleJourney) -> Option<&str> {
    journey
        .destination_name
        .first()
        .and_then(|localized| localized.value.as_deref())
}

pub fn get_line_ref(journey: &MonitoredVehicleJourney) -> Option<&str> {
    journey
        .line_ref
        .as_ref()
        .and_then(|ref_value| ref_value.value.as_deref())
}

// c'est beau hein, merci IDFM ;)
pub fn localized_line_name(
    journey: &MonitoredVehicleJourney,
) -> Option<&'static str> {
    let line_ref = get_line_ref(journey)?;
    match line_ref {
        "STIF:Line::C01742:" => Some("A"),
        "STIF:Line::C01743:" => Some("B"),
        "STIF:Line::C01727:" => Some("C"),
        "STIF:Line::C01728:" => Some("D"),
        "STIF:Line::C01729:" => Some("E"),
        _ => None,
    }
}

use crate::point::Point;
use crate::waypoints::Waypoint;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Builder)]
pub struct RouteRequest {
    pub points: Vec<Point>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SimpleRouteResponse {
    pub code: String,
    pub durations: f64,
    pub distance: f64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RouteResponse {
    pub code: String,
    pub routes: Vec<Route>,
    pub waypoints: Vec<Waypoint>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OsrmResponse {
    pub code: String,
    pub routes: Vec<Route>,
    pub waypoints: Vec<Waypoint>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Route {
    pub legs: Vec<Leg>,
    pub weight_name: String,
    pub geometry: Geometry,
    pub weight: f64,
    pub duration: f64,
    pub distance: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Leg {
    pub steps: Vec<Step>,
    pub weight: f64,
    pub summary: String,
    pub duration: f64,
    pub distance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub coordinates: Vec<[f64; 2]>,
    #[serde(rename = "type")]
    pub geometry_type: GeometryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GeometryType {
    #[serde(rename = "LineString")]
    LineString,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    pub intersections: Vec<Intersection>,
    pub maneuver: Maneuver,
    pub name: String,
    pub duration: f64,
    pub distance: f64,
    pub driving_side: DrivingSide,
    pub weight: f64,
    pub mode: Mode,
    pub geometry: Geometry,
    #[serde(rename = "ref")]
    pub step_ref: Option<String>, // existed in both (already optional)
    pub destinations: Option<String>, // only in second → optional
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrivingSide {
    Left,
    Right,
    Straight,
    // only present in some version(s) / strings:
    #[serde(rename = "slight left")]
    SlightLeft,
    #[serde(rename = "slight right")]
    SlightRight,
    // only in second → include to be a superset:
    None,
    Uturn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Intersection {
    pub entry: Vec<bool>,
    pub bearings: Vec<i64>,
    pub location: [f64; 2],
    #[serde(rename = "in")]
    pub intersection_in: Option<i64>,

    // present in both but optional in at least one:
    pub out: Option<i64>,
    pub lanes: Option<Vec<Lane>>,
    pub classes: Option<Vec<String>>,

    // only in first → make optional:
    pub duration: Option<f64>,
    pub admin_index: Option<i64>,
    pub weight: Option<f64>,
    pub geometry_index: Option<i64>,
    pub turn_weight: Option<f64>,
    pub turn_duration: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lane {
    pub indications: Vec<DrivingSide>,
    pub valid: bool,

    // only in first → optional:
    pub active: Option<bool>,
    pub valid_indication: Option<DrivingSide>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Maneuver {
    pub bearing_after: i64,
    pub bearing_before: i64,
    pub location: [f64; 2],
    pub modifier: Option<DrivingSide>,
    #[serde(rename = "type")]
    pub maneuver_type: ManeuverType,

    // only in second → optional:
    pub exit: Option<i64>,
    // only in first → optional:
    pub instruction: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManeuverType {
    Arrive,
    Continue,
    Depart,
    #[serde(rename = "end of road")]
    EndOfRoad,
    Fork,
    Turn,

    // only in second → include to be a superset:
    Merge,
    #[serde(rename = "new name")]
    NewName,
    #[serde(rename = "on ramp")]
    OnRamp,
    #[serde(rename = "roundabout turn")]
    RoundaboutTurn,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Driving,
}

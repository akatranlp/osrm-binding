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
    #[serde(rename = "slight left")]
    SlightLeft,
    #[serde(rename = "slight right")]
    SlightRight,
    #[serde(rename = "sharp left")]
    SharpLeft,
    #[serde(rename = "sharp right")]
    SharpRight,
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
    /// a basic turn into direction of the modifier
    Turn,
    /// no turn is taken/possible, but the road name changes. The road can take a turn itself, following modifier .
    #[serde(rename = "new name")]
    NewName,
    /// indicates the departure of the leg
    Depart,
    /// indicates the destination of the leg
    Arrive,
    /// merge onto a street (e.g. getting on the highway from a ramp, the modifier specifies the direction of the merge )
    Merge,
    /// take a ramp to enter a highway (direction given my modifier )
    #[serde(rename = "on ramp")]
    OnRamp,
    /// take a ramp to exit a highway (direction given my modifier )
    #[serde(rename = "off ramp")]
    OffRamp,
    /// take the left/right side at a fork depending on modifier
    Fork,
    /// road ends in a T intersection turn in direction of modifier
    #[serde(rename = "end of road")]
    EndOfRoad,
    /// Turn in direction of modifier to stay on the same road
    Continue,
    /// traverse roundabout, if the route leaves the roundabout there will be an additional property exit for exit counting. The modifier specifies the direction of entering the roundabout.
    Roundabout,
    /// a traffic circle. While very similar to a larger version of a roundabout, it does not necessarily follow roundabout rules for right of way. It can offer rotary_name and/or rotary_pronunciation parameters (located in the RouteStep object) in addition to the exit parameter (located on the StepManeuver object).
    Rotary,
    /// Describes a turn at a small roundabout that should be treated as normal turn. The modifier indicates the turn direciton. Example instruction: At the roundabout turn left .
    #[serde(rename = "roundabout turn")]
    RoundaboutTurn,
    /// not an actual turn but a change in the driving conditions. For example the travel mode or classes. If the road takes a turn itself, the modifier describes the direction
    Notification,
    /// Describes a maneuver exiting a roundabout (usually preceeded by a roundabout instruction)
    #[serde(rename = "exit roundabout")]
    ExitRoundabout,
    /// Describes the maneuver exiting a rotary (large named roundabout)
    #[serde(rename = "exit rotary")]
    ExitRotary,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Driving,
}

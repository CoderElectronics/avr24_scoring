use derivative::Derivative;
use uuid::Uuid;

#[derive(Clone, Derivative)]
#[derivative(Debug, Default)]
pub struct ScoringAction {
    #[derivative(Default(value = "Uuid::new_v4().to_string()"))]
    pub id: String,

    #[derivative(Default(value = "String::from(\"N/A\")"))]
    pub description: String,
    pub name: String,
    pub phase: i8,

    pub pointvalue: i8, // set to -1 to use point stages instead

    #[derivative(Default(value = "10"))]
    pub max_count: i8, // if using point stages this is useless

    #[derivative(Default(value = "0"))]
    pub count: i8, // if using point stages this is the index of the pointstages vector

    #[derivative(Default(value = "Vec::new()"))]
    pub pointstages: Vec<i8>,
}

pub fn initialize_default_actions() -> Vec<ScoringAction> {
    let mut ret = Vec::new();

    // Phase 1
    let phase_1 = ScoringAction {
        name: "Phase 1".to_string(),
        description: "Score in the goal zone".to_string(),
        pointvalue: 2,
        phase: 1,
        ..Default::default()
    };
    ret.push(phase_1);

    // Phase 2
    let phase_2 = ScoringAction {
        name: "Phase 2".to_string(),
        description: "Score in the railway".to_string(),
        pointvalue: 4,
        phase: 2,
        ..Default::default()
    };
    ret.push(phase_2);

    // Phase 3
    let phase_3 = ScoringAction {
        name: "Phase 3".to_string(),
        description: "Score in the hotspots".to_string(),
        pointvalue: 8,
        phase: 3,
        ..Default::default()
    };
    ret.push(phase_3);

    // Phase 4
    let p4_parking_rvr_avr = ScoringAction {
        name: "General parking".to_string(),
        description: "AVR, RVR, and DEXI has parked.".to_string(),
        pointvalue: 3,
        phase: 4,
        max_count: 1,
        ..Default::default()
    };
    ret.push(p4_parking_rvr_avr);

    let p4_parking_minis = ScoringAction {
        name: "Mini parking".to_string(),
        description: "This number of sphero mini drivers that have parked.".to_string(),
        pointvalue: 1,
        phase: 4,
        max_count: 3,
        ..Default::default()
    };
    ret.push(p4_parking_minis);

    let p4_stacking = ScoringAction {
        name: "Stacking".to_string(),
        description: "Creating a single unsupported stack of conex boxes with AVR. Enter height."
            .to_string(),
        pointvalue: -1,
        phase: 4,
        pointstages: vec![0, 0, 1, 3, 5, 7, 20],
        ..Default::default()
    };
    ret.push(p4_stacking);

    return ret;
}

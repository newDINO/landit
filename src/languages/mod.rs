mod chinese;
pub use chinese::CHINESE;

pub struct LangDict {
    pub dashboard: &'static str,
    pub thrust: &'static str,
    pub angle: &'static str,
    pub chopstick_up_down: &'static str,

    pub settings: &'static str,
    pub debug: &'static str,
    pub draw_transforms: &'static str,
    pub camera: &'static str,
    pub control_camera_style1: &'static str,
}

pub const ENGLISH: LangDict = LangDict {
    dashboard: "DashBoard",
    thrust: "Thrust",
    angle: "Angle",
    chopstick_up_down: "Chopstick Up Down",
    settings: "Settings",
    debug: "Debug",
    draw_transforms: "Draw Transforms",
    camera: "Camera",
    control_camera_style1: "Control Camera Style1",
};

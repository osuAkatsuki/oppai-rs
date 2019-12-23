pub mod timing {
    // The timing point of a beatmap object.
    pub struct BaseTimingPoint {
        pub time: f32,
        pub ms_per_beat: f32,
        pub change: i32,
    }

    pub type TimingPoint = BaseTimingPoint;

    // Taiko-specific timing point.
    pub struct TaikoTimingPoint {
        pub base: BaseTimingPoint,
        pub beat_len: f32,
        pub velocity: f32,
    }
}

pub mod object {
    // The base of a beatmap object.
    pub struct BaseObject {
        pub time: f32,

        // Used by difficulty calculator
        pub norm_pos: (f32, f32),
        pub angle: f32,
        pub strains: (f32, f32),
        pub is_single: bool, // if diff calc sees this as a singletap
        pub delta_time: f32,
        pub d_distance: f32,
        pub timing_point: i32,

        pub pos: (f32, f32),
    }

    // A slider object.
    pub struct Slider {
        pub base: BaseObject,

        pub distance: f32,
        pub repetitions: u32,
    }

    // A beatmap object, specialized on the type of the object.
    pub enum Object {
        Circle(BaseObject),
        Slider(Slider),
        Spinner(BaseObject),
    }

    // A special object for a taiko map.
    pub struct TaikoObject {
        pub base: Object,

        pub sound_types: Vec<u32>,

        pub duration: f32,
        pub tick_spacing: f32,
        pub slider_is_drum_roll: bool,
    }
}

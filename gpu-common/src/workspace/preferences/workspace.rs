crate::config_category! {
    struct Workspace {
        /// The size of the pane drag handle.
        handle_size: {
            Self = u8,
            class = Int,
            min = 0,
            max = 12,
            postfix = "px",
        },
        ui_speed: UiSpeed,
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self {
            handle_size: 3,
            ui_speed: Default::default(),
        }
    }
}

crate::config_enum! {
    /// The speed of ui transitions.
    #[derive(Default)]
    enum UiSpeed {
        ["instant"] => Instant,
        ["snappy"] => Snappy,
        #[default]
        ["quick"] => Quick,
        ["smooth"] => Smooth,
        ["slow"] => Slow,
    }
}

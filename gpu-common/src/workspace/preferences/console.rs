crate::config_category! {
    #[derive(Default)]
    struct Console {
        show_completions: ShowCompletions,
        /// True: Wrap console messages. May scramble certain error messages.
        /// False: Console messages overflow past the size of the viewport.
        wrap: {
            Self = bool,
            class = BoolClass,
        },
        level: LogLevel,
    }
}

crate::config_enum! {
    /// How much detail should completions provide.
    #[derive(Default)]
    enum ShowCompletions {
        #[default]
        ["full"] => Full,
        ["suggestions"] => Suggestions,
        ["none"] => None,
    }
}

crate::config_enum! {
    /// Log level of the console.
    #[derive(Default)]
    enum LogLevel {
        ["trace"] => Trace,
        ["debug"] => Debug,
        #[default]
        ["info"] => Info,
        ["warn"] => Warn,
        ["error"] => Error,
    }

}

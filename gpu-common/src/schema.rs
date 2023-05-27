use crate::{
    api::{
        Credentials, LoginResponse, NewUser, NewUserResponse, ProjectResponse, ProjectUpsert,
        UpdateUserInfoArgs, UserInfoResponse,
    },
    completion::CompletionEntry,
    config_value::ConfigValueSchema,
    describe::CompletionInfo,
    Action, ClientError, Config, PrebuildResult, Preferences, Project, Runner,
};

macro_rules! root_schema {
    ($($type:ty $(: $lt:lifetime)?,)*) => {
        ::paste::paste! {
            #[derive(schemars::JsonSchema)]
            #[allow(dead_code)]
            pub struct RootSchema<'all> {
                $(
                    [<$type:lower>]: $type$(<$lt>)?,
                )*
            }
        }
    };
}

root_schema!(
    Project,
    Config,
    ProjectUpsert,
    ProjectResponse,
    NewUser,
    NewUserResponse,
    Credentials,
    LoginResponse,
    UserInfoResponse,
    UpdateUserInfoArgs,
    Action,
    PrebuildResult,
    Runner,
    ClientError,
    Preferences,
    ConfigValueSchema,
    CompletionInfo: 'all,
    CompletionEntry,
);

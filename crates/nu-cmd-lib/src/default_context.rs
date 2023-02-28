use nu_protocol::engine::{EngineState, StateWorkingSet};

use crate::*;

pub fn create_default_context() -> EngineState {
    let mut engine_state = EngineState::new();

    let delta = {
        let mut working_set = StateWorkingSet::new(&engine_state);

        macro_rules! bind_command {
            ( $( $command:expr ),* $(,)? ) => {
                $( working_set.add_decl(Box::new($command)); )*
            };
        }

        // Env
        bind_command! {
            Env,
            ExportEnv,
            LetEnv,
            LoadEnv,
            SourceEnv,
            WithEnv,
            ConfigNu,
            ConfigEnv,
            ConfigMeta,
            ConfigReset,
        };

        // System
        bind_command! {
            Complete,
            External,
            NuCheck,
            Sys,
        };

        working_set.render()
    };

    if let Err(err) = engine_state.merge_delta(delta) {
        eprintln!("Error creating default context: {err:?}");
    }

    engine_state
}

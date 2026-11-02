//! Shared vocabulary for the CLI's mutation command grammar: subcommand and option
//! tokens, and canonical positional order. `main.rs`'s Clap declarations and the
//! preview module's replay builder both read from these constants, so the binary's
//! actual grammar and a rendered replay invocation can never drift into two
//! different spellings of the same command.

/// Binary name as invoked from a shell.
pub const BINARY: &str = "erpnext";

pub const COMPANY: &str = "company";
pub const SITE: &str = "site";
pub const AS: &str = "as";
pub const DATA: &str = "data";
pub const SOURCE: &str = "source";
pub const VIA: &str = "via";
pub const ARGS: &str = "args";
pub const APPLY: &str = "apply";

/// Positional value names. `METHOD` names both the controller-method positional
/// argument and the `workflow method` subcommand token: the same word, one meaning.
pub const DOCTYPE: &str = "doctype";
pub const NAME: &str = "name";
pub const METHOD: &str = "method";

pub const DOC: &str = "doc";
pub const WORKFLOW: &str = "workflow";
pub const CREATE: &str = "create";
pub const UPDATE: &str = "update";
pub const DELETE: &str = "delete";
pub const COPY: &str = "copy";
pub const MAP: &str = "map";
pub const SUBMIT: &str = "submit";
pub const CANCEL: &str = "cancel";
pub const AMEND: &str = "amend";
pub const CALL: &str = "call";

/// The ten mutating document, workflow, and native-method subcommands that preview by default. A
/// closed type: every replay and every Clap declaration names one of these, never
/// a bare string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationCommand {
    DocCreate,
    DocUpdate,
    DocDelete,
    DocCopy,
    DocMap,
    WorkflowSubmit,
    WorkflowCancel,
    WorkflowAmend,
    WorkflowMethod,
    MethodCall,
}

impl MutationCommand {
    /// The `doc`/`workflow` group token and the subcommand token, in that order.
    pub fn tokens(self) -> [&'static str; 2] {
        match self {
            Self::DocCreate => [DOC, CREATE],
            Self::DocUpdate => [DOC, UPDATE],
            Self::DocDelete => [DOC, DELETE],
            Self::DocCopy => [DOC, COPY],
            Self::DocMap => [DOC, MAP],
            Self::WorkflowSubmit => [WORKFLOW, SUBMIT],
            Self::WorkflowCancel => [WORKFLOW, CANCEL],
            Self::WorkflowAmend => [WORKFLOW, AMEND],
            Self::WorkflowMethod => [WORKFLOW, METHOD],
            Self::MethodCall => [METHOD, CALL],
        }
    }

    /// Positional argument names in the exact order the binary's Clap declaration
    /// takes them for this subcommand. Replay emits values in this order.
    pub fn positionals(self) -> &'static [&'static str] {
        match self {
            Self::DocCreate => &[DOCTYPE],
            Self::DocUpdate
            | Self::DocDelete
            | Self::DocCopy
            | Self::DocMap
            | Self::WorkflowSubmit
            | Self::WorkflowCancel
            | Self::WorkflowAmend => &[DOCTYPE, NAME],
            Self::WorkflowMethod => &[DOCTYPE, NAME, METHOD],
            Self::MethodCall => &[METHOD],
        }
    }
}

/// Quote one token for a POSIX shell: wrap it in single quotes and escape any
/// embedded single quote as `'\''`. This is the one algorithm that stays correct
/// for spaces, double quotes, newlines, `$`, backticks, and the empty string,
/// without executing anything itself.
pub fn shell_quote(token: &str) -> String {
    let mut quoted = String::with_capacity(token.len() + 2);
    quoted.push('\'');
    for ch in token.chars() {
        if ch == '\'' {
            quoted.push_str("'\\''");
        } else {
            quoted.push(ch);
        }
    }
    quoted.push('\'');
    quoted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_quote_preserves_metacharacters_and_empty_input() {
        assert_eq!(shell_quote(""), "''");
        assert_eq!(shell_quote("plain"), "'plain'");
        assert_eq!(shell_quote("a'b"), "'a'\\''b'");
        assert_eq!(shell_quote("$(rm -rf /)"), "'$(rm -rf /)'");
        assert_eq!(shell_quote("line\nbreak"), "'line\nbreak'");
    }

    #[test]
    fn mutation_command_tokens_and_positionals_are_consistent() {
        assert_eq!(MutationCommand::DocCreate.tokens(), [DOC, CREATE]);
        assert_eq!(MutationCommand::DocCreate.positionals(), &[DOCTYPE]);
        assert_eq!(
            MutationCommand::WorkflowMethod.positionals(),
            &[DOCTYPE, NAME, METHOD]
        );
    }
}

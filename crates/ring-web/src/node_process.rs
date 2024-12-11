use crate::ScriptFile;
use regex::Regex;
use ring_code_unit::CodeUnit;
use ring_color::ColorLabel;
use ring_process_unit::ProcessUnit;
use ring_tag::{Tag, Tagged};
use std::rc::Rc;
use std::sync::OnceLock;
use sysinfo::Pid;

// Parameters
static SHOULD_HIDE_REGEX: OnceLock<[Regex; 2]> = OnceLock::new();

fn should_hide_regex() -> &'static [Regex; 2] {
    SHOULD_HIDE_REGEX.get_or_init(|| [
        Regex::new(r"yarn\.[cm]?js$").unwrap(),
        Regex::new(r"yarn-([0-9]+\.){3}[cm]?js$").unwrap(),
    ])
}

////////////////////////////////////////////////////////////////////////////////
// Node Process
////////////////////////////////////////////////////////////////////////////////

/// Represent a process running a script file using node
pub struct NodeProcess {
    pid: Pid,
    arguments: Vec<String>,
    running_script: Option<Rc<ScriptFile>>,
}

impl NodeProcess {
    pub fn new(pid: Pid, arguments: Vec<String>, running_script: Option<Rc<ScriptFile>>) -> NodeProcess {
        NodeProcess { pid, arguments, running_script }
    }

    pub fn running_script(&self) -> Option<&Rc<ScriptFile>> {
        self.running_script.as_ref()
    }
}

impl ProcessUnit for NodeProcess {
    fn cmd(&self) -> Vec<&str> {
        let mut cmd = vec![];

        if let Some(script) = &self.running_script {
            cmd.push(script.name().unwrap());
        }

        cmd.extend(self.arguments.iter().map(|s| s.as_str()));

        cmd
    }

    fn pid(&self) -> &Pid {
        &self.pid
    }

    fn running_code_unit(&self) -> Option<Rc<dyn CodeUnit>> {
        self.running_script.clone()
            .map(|script| script as Rc<dyn CodeUnit>)
    }

    fn should_hide(&self) -> bool {
        self.running_script.clone()
            .is_some_and(|script| {
                let path = script.path().to_str().unwrap();
                should_hide_regex().iter().any(|re| re.is_match(path))
            })
    }
}

impl Tagged for NodeProcess {
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::from("node").with_color(ColorLabel::Green, (0x5f, 0xa0, 0x4e))]
    }
}
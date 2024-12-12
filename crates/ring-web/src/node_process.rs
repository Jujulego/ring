use crate::ScriptFile;
use regex::Regex;
use ring_color::ColorLabel;
use ring_core::{CodeUnit, ProcessUnit};
use ring_tag::{Tag, Tagged};
use std::rc::Rc;
use std::sync::OnceLock;
use sysinfo::Pid;

// Parameters
const HIDE_PACKAGES: [&str; 2] = ["npm", "yarn"];
static HIDE_SCRIPT_REGEX: OnceLock<[Regex; 3]> = OnceLock::new();

fn hide_script_regex() -> &'static [Regex; 3] {
    HIDE_SCRIPT_REGEX.get_or_init(|| [
        Regex::new(r"yarn-([0-9]+\.){3}[cm]?js$").unwrap(),
        Regex::new(r"WebStorm[/\\]plugins[/\\]javascript-plugin[/\\]jsLanguageServicesImpl[/\\]").unwrap(),
        Regex::new(r"typescript[/\\]lib[/\\]typingsInstaller\.[cm]?js$").unwrap(),
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
        if let Some(script) = &self.running_script {
            // Hide some packages
            let package = script.package()
                .and_then(|pkg| pkg.name());

            if package.is_some_and(|name| HIDE_PACKAGES.contains(&name)) {
                return true;
            }

            // Hide some scripts
            let path = script.path().to_str().unwrap();
            hide_script_regex().iter().any(|re| re.is_match(path))
        } else {
            false
        }
    }
}

impl Tagged for NodeProcess {
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::from("node").with_color(ColorLabel::Green, (0x5f, 0xa0, 0x4e))]
    }
}
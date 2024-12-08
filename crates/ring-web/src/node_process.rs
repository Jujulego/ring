use crate::ScriptFile;
use ring_code_unit::CodeUnit;
use ring_color::ColorLabel;
use ring_process_unit::ProcessUnit;
use ring_tag::{Tag, Tagged};
use std::rc::Rc;
use regex::Regex;
use sysinfo::Pid;

pub struct NodeProcess {
    pid: Pid,
    arguments: Vec<String>,
    running_script: Rc<ScriptFile>,
}

impl NodeProcess {
    pub fn new(pid: Pid, arguments: Vec<String>, running_script: Rc<ScriptFile>) -> NodeProcess {
        NodeProcess { pid, arguments, running_script }
    }

    pub fn running_script(&self) -> &Rc<ScriptFile> {
        &self.running_script
    }
}

impl ProcessUnit for NodeProcess {
    fn cmd(&self) -> Vec<&str> {
        let mut cmd = vec![self.running_script.name().unwrap(), ];
        cmd.extend(self.arguments.iter().map(|s| s.as_str()));

        cmd
    }

    fn pid(&self) -> &Pid {
        &self.pid
    }

    fn running_code_unit(&self) -> Option<Rc<dyn CodeUnit>> {
        Some(self.running_script.clone())
    }

    fn should_hide(&self) -> bool {
        let re = Regex::new(r"^yarn-([0-9]+\.){3}[cm]?js$").unwrap();

        self.running_script.name().is_some_and(|name| {
            name == "yarn.js" || re.is_match(name)
        })
    }
}

impl Tagged for NodeProcess {
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::from("node").with_color(ColorLabel::Green, (0x5f, 0xa0, 0x4e))]
    }
}
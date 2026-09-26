pub enum NodeRemovalResult {
    ChildrenCleared,
    PIDDeletedFromChild,
    PIDNotFound,
}

#[derive(Debug, Clone)]
pub struct PidNode {
    pub pid: u32,
    pub comm: String,
    pub cmdline: String,
    pub children: Vec<PidNode>,
    pub child_count: usize,
}

impl PidNode {
    pub fn new(pid: u32, comm: String, cmdline: String, children: Vec<PidNode>) -> Self {
        Self {
            pid,
            comm,
            cmdline,
            children,
            child_count: 0,
        }
    }

    pub fn add_child(
        &mut self,
        pid: u32,
        comm: String,
        cmdline: String,
        mut children: Vec<PidNode>,
    ) {
        let child = Self::new(pid, comm, cmdline, children);
        self.children.push(child);
        self.child_count += 1;
    }

    pub fn find_and_add_child(
        &mut self,
        ppid: u32,
        pid: u32,
        comm: &String,
        cmdline: &String,
        children: &Vec<PidNode>,
    ) -> bool {
        // all the children and find the appropriate
        // place to insert the new child
        // its recursive function
        if self.pid == ppid {
            self.add_child(pid, comm.to_string(), cmdline.to_string(), children.clone());
            return true;
        } else if self.child_count > 0 {
            for child in &mut self.children {
                let value = child.find_and_add_child(ppid, pid, &comm, &cmdline, &children);
                if value {
                    return true;
                }
            }
        }
        false
    }

    pub fn remove_child(&mut self, pid: u32) -> NodeRemovalResult {
        if self.pid == pid {
            self.children.clear();
            return NodeRemovalResult::ChildrenCleared;
        } else {
            for child in &mut self.children {
                let value: NodeRemovalResult = child.remove_child(pid);
                match value {
                    NodeRemovalResult::ChildrenCleared => {
                        // delete the child from the vector
                        self.children.retain(|c| c.pid != pid);
                        self.child_count -= 1;
                        return NodeRemovalResult::PIDDeletedFromChild;
                    }
                    NodeRemovalResult::PIDDeletedFromChild => {
                        return NodeRemovalResult::PIDNotFound;
                    }
                    NodeRemovalResult::PIDNotFound => {
                        continue;
                    }
                }
            }
        }
        NodeRemovalResult::PIDNotFound
    }

    pub fn find(&self, pid: u32) -> bool {
        if self.pid == pid {
            return true;
        }
        for child in &self.children {
            if child.find(pid) {
                return true;
            }
        }
        false
    }

    pub fn print(&self, pid: Option<u32>, print_layer: u32) {
        match pid {
            Some(pid) => {
                if self.pid == pid {
                    if print_layer == 0 {
                        println!("|----{}({})", self.pid, self.comm);
                    } else {
                        let tab = "|----".repeat(print_layer as usize);
                        println!("{}|----{}({})", tab, self.pid, self.comm);
                    }
                    for child in &self.children {
                        child.print(None, print_layer + 1);
                    }
                }
            }
            None => {
                if print_layer == 0 {
                    println!("|----{}({})", self.pid, self.comm);
                } else {
                    let tab = "|----".repeat(print_layer as usize);
                    println!("{}|----{}({})", tab, self.pid, self.comm);
                }
                for child in &self.children {
                    child.print(None, print_layer + 1);
                }
            }
        }
    }
}

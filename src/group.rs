fn start_group(title: &str) {
    println!(
        "{}",
        crate::Command {
            command: "group",
            value: title,
            properties: None
        }
    );
}

fn end_group() {
    println!(
        "{}",
        crate::Command {
            command: "endgroup",
            value: "",
            properties: None
        }
    );
}

pub struct Group;

impl Group {
    pub fn new(title: &str) -> Self {
        start_group(title);
        Self {}
    }

    pub fn end(&self) {
        end_group();
    }
}

pub struct GroupEnder<'a> {
    group: &'a Group,
}

impl<'a> GroupEnder<'a> {
    pub fn new(group: &'a Group) -> Self {
        Self { group }
    }
}

impl<'a> Drop for GroupEnder<'a> {
    fn drop(&mut self) {
        self.group.end();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        {
            let group = Group::new("group 1");
            let _group_ender = GroupEnder::new(&group);
        }
        {
            let _group = Group::new("group 2");
            // it doesn't end
        }
    }
}

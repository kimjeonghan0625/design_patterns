type FnPtr = fn() -> String;

struct Command {
    execute: FnPtr,
    rollback: FnPtr,
}

struct Schema {
    commands: Vec<Command>,
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
        self.commands.push(Command { execute, rollback });
    }

    fn execute(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }

    fn rollback(&self) -> Vec<String> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

fn add_field() -> String {
    "add field".to_string()
}

fn remove_field() -> String {
    "remove field".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_migrations() {
        let mut schema = Schema::new();
        schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
        schema.add_migration(add_field, remove_field);

        let execute_results = schema.execute();
        assert_eq!(execute_results, vec!["create table", "add field"]);

        let rollback_results = schema.rollback();
        assert_eq!(rollback_results, vec!["remove field", "drop table"]);
    }
}

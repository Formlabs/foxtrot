use crate::ap214_autogen::DataEntity;

////////////////////////////////////////////////////////////////////////////////

pub struct StepFile<'a>(pub Vec<DataEntity<'a>>);

impl StepFile<'_> {
    pub fn to_dot(&self) -> String {
        let mut out = "digraph {\n".to_owned();
        for (i, e) in self.0.iter().enumerate() {
            let d = format!("{:?}", e);
            let name = d.split("(").next().unwrap();

            out += &format!("  e{} [ label = \"#{}: {}\" ];\n", i, i, name);
            for j in e.upstream() {
                out += &format!("  e{} -> e{};\n", i, j.0);
            }
        }
        out += "}";
        out
    }
    pub fn save_dot(&self, filename: &str) -> std::io::Result<()> {
        std::fs::write(filename, self.to_dot())
    }
}

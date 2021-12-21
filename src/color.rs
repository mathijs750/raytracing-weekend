use crate::vec3::*;

impl Color {
    pub fn write_color(self) {
        print!("{} {} {}\n", self.x, self.y, self.z);
    }
}

use crate::vec3::Vec3;
pub type Color = Vec3;

impl Color {
    pub fn write_color(self) {
        print!("{} {} {}\n", self.x, self.y, self.z);
    }
}

use crate::vec3::*;

impl Color {
    pub fn write_color(self) {
        print!(
            "{} {} {}\n",
            (self.x * 255.999).round(),
            (self.y * 255.999).round(),
            (self.z * 255.999).round()
        );
    }
}

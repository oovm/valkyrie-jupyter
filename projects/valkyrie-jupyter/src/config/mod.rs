
pub struct ValkyrieConfig {
    running_time: bool,
    image_max_pixel: usize,
}

impl Default for ValkyrieConfig {
    fn default() -> Self {
        ValkyrieConfig { running_time: false, image_max_pixel: 1024 * 768 }
    }
}

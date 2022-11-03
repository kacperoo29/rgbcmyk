use nalgebra_glm::{Vec3, Mat4x4};

pub struct Camera {
    position: Vec3,
    front: Vec3,
    up: Vec3,
    right: Vec3,
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,

    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new() -> Self {
        let front = Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let right = nalgebra_glm::cross(&front, &up).normalize();

        Self {
            position: Vec3::new(0.0, 0.0, 1.5),
            front: front,
            up: up,
            right: right,
            fov: 90.0f32.to_radians(),
            aspect: 4.0 / 3.0,
            near: 0.1,
            far: 100.0,
            yaw: -90.0,
            pitch: 0.0,
        }
    }

    pub fn calculate_view_matrix(&self) -> Mat4x4 {
        nalgebra_glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn calculate_projection_matrix(&self) -> Mat4x4 {
        nalgebra_glm::perspective(self.aspect, self.fov, self.near, self.far)
    }

    pub fn move_dir(&mut self, direction: Vec3) {       
        let velocity = direction.normalize() * 0.05;

        self.position += velocity.x * self.right;
        self.position += velocity.y * self.up;
        self.position += velocity.z * self.front;
    }

    pub fn rotate(&mut self, dx: f32, dy: f32) {
        self.yaw += dx;
        self.pitch += dy;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        self.front = nalgebra_glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        )
        .normalize();

        self.right =
            nalgebra_glm::cross(&self.front, &nalgebra_glm::vec3(0.0, 1.0, 0.0)).normalize();
        self.up = nalgebra_glm::cross(&self.right, &self.front).normalize();
    }
}

use nalgebra_glm as glm;
use crate::utils;

pub enum CameraMovement {
    Fwd,
    Bwd,
    Left,
    Right
}
pub struct Camera {
    // The camera's position in world-space
    worldspace_position: glm::Vec3,
    // The positive Y axis in world-space
    worldspace_up: glm::Vec3,

    // These three variables define a new orthogonal basis of R3, with the camera's position in world-space as the origin
    // camera_direction faces towards where the camera is pointing (and thus, the vector that traces in the direction the user can see)
    camera_direction: glm::Vec3,
    // camera_right is orthogonal to camera_direction and world-space up.
    camera_right: glm::Vec3,
    // camera_up is orthogonal to both camera_direction and camera_right, meaning it is the normal vector to the plane defined by camera_direction and camera_right
    camera_up: glm::Vec3,

    // Euler angles for pitch/yaw
    pitch: f32,
    yaw: f32,

    // We're leaving out the ability to change the movement speed of the camera.
    // Some other component should handle movement

    // Should mouse_sensitivity be local to this camera? or should there be a composable component that patches in the mouse movement, since not all cameras need mouse movement?
    // I would lean towards having a second camera structure that has mouse_sensitivity that is referenced while it is the active camera, since mouse movements do not cause an effect on the game-world (except wherein a head model would move)
    mouse_sensitivity: f32,

    // the FOV passed into the camera. This might not be the best place for it; since this is used for the projection which is typically per-display.
    // However, there might be merit in calculating the projection matrix per camera; this would allow for different FOV's and different aspect ratios per camera.
    // Food for thought, esp. considering only the active camera object should be presenting it's output matrix (i.e. the camera system should only be calculating one VP matrix per-frame)
    fov: f32,
}

impl Camera {
    pub fn new(
        worldspace_position: glm::Vec3,
        worldspace_up: glm::Vec3,
        initial_direction: glm::Vec3,
        initial_pitch: f32,
        initial_yaw: f32,
        initial_mouse_sensitivity: f32,
        initial_fov: f32,
    ) -> Camera {
        let cam_right = glm::normalize(&glm::cross(&initial_direction, &worldspace_up));
        let cam_up = glm::normalize(&glm::cross(&cam_right, &initial_direction));
        Camera {
            worldspace_position: worldspace_position,
            worldspace_up: worldspace_up,
            camera_direction: initial_direction,
            camera_right: cam_right,
            camera_up: cam_up,
            pitch: initial_pitch,
            yaw: initial_yaw,
            mouse_sensitivity: initial_mouse_sensitivity,
            fov: initial_fov,
        }
    }

    pub fn process_movement(&mut self, direction: CameraMovement, delta_t: f32) {
        let velocity = super::SPEED * delta_t;
        self.worldspace_position = 
            match direction {
                CameraMovement::Fwd => self.worldspace_position + (self.camera_direction * velocity),
                CameraMovement::Bwd => self.worldspace_position - (self.camera_direction * velocity),
                CameraMovement::Right => self.worldspace_position + (self.camera_right * velocity),
                CameraMovement::Left => self.worldspace_position - (self.camera_right * velocity),
            };
    }

    pub fn process_mouse_input(&mut self, x_off: f32, y_off: f32, constrain_pitch: bool) {
        let cor_x_off = x_off * self.mouse_sensitivity;
        let cor_y_off = y_off * self.mouse_sensitivity;

        self.yaw += cor_x_off;
        self.pitch += cor_y_off;

        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }

            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        self.update_internal_cam_vectors();
    }

    pub fn update_internal_cam_vectors(&mut self) {
        let direction = glm::vec3(
            utils::radians(self.yaw).cos() * utils::radians(self.pitch).cos(),
            utils::radians(self.pitch).sin(),
            utils::radians(self.yaw).sin() * utils::radians(self.pitch).cos()
        );
        let norm_dir = glm::normalize(&direction);
        let cam_right = glm::normalize(&glm::cross(&norm_dir, &self.worldspace_up));
        let cam_up = glm::normalize(&glm::cross(&cam_right, &norm_dir));

        self.camera_direction = norm_dir;
        self.camera_right = cam_right;
        self.camera_up = cam_up;
    }

    pub fn generate_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.worldspace_position, &(self.worldspace_position + self.camera_direction), &self.camera_up)
    }

    pub fn generate_projection_matrix(&self, width: f32, height: f32) -> glm::Mat4 {
        glm::perspective(width / height, utils::radians(self.fov), 0.1, 100.0)
    }
}
use super::level_transform::LevelTransform;
use super::site_map::{Editable, Handles};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::PickableBundle;
use serde_yaml;

#[derive(serde::Deserialize, Component, Inspectable, Clone, Default)]
#[serde(try_from = "VertexRaw")]
pub struct Vertex {
    pub x_raw: f64,
    pub y_raw: f64,
    pub x_meters: f64,
    pub y_meters: f64,
    pub _name: String,
}

impl TryFrom<VertexRaw> for Vertex {
    type Error = String;

    fn try_from(raw: VertexRaw) -> Result<Vertex, Self::Error> {
        let x_raw = raw.data[0]
            .as_f64()
            .ok_or("expected first element to be a number")?;
        let y_raw = raw.data[1]
            .as_f64()
            .ok_or("expected second element to be a number")?;
        let name = if raw.data.len() > 3 {
            raw.data[3]
                .as_str()
                .ok_or("expected third element to be a string")?
                .to_string()
        } else {
            String::new()
        };
        Ok(Vertex {
            x_raw,
            y_raw: -y_raw,
            x_meters: x_raw,
            y_meters: -y_raw,
            _name: name,
        })
    }
}

impl Vertex {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        handles: &Res<Handles>,
        transform: &LevelTransform,
    ) {
        commands
            .spawn_bundle(PbrBundle {
                mesh: handles.vertex_mesh.clone(),
                material: handles.vertex_material.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        self.x_meters as f32,
                        self.y_meters as f32,
                        transform.translation[2] as f32,
                    ),
                    rotation: Quat::from_rotation_x(1.57),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(PickableBundle::default())
            .insert(Editable::Vertex(self.clone()));
    }

    pub fn from_yaml(value: &serde_yaml::Value) -> Vertex {
        let data = value.as_sequence().unwrap();
        let x_raw = data[0].as_f64().unwrap();
        let y_raw = data[1].as_f64().unwrap();
        let name = if data.len() > 3 {
            data[3].as_str().unwrap().to_string()
        } else {
            String::new()
        };
        return Vertex {
            x_raw: x_raw,
            y_raw: -y_raw,
            x_meters: x_raw,
            y_meters: -y_raw,
            _name: name,
        };
    }
}

#[derive(serde::Deserialize)]
#[serde(transparent)]
struct VertexRaw {
    data: Vec<serde_yaml::Value>,
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct VertexProperties {
    is_charger: Option<(usize, bool)>,
    is_parking_spot: Option<(usize, bool)>,
    is_holding_point: Option<(usize, bool)>,
    spawn_robot_name: Option<(usize, String)>,
    spawn_robot_type: Option<(usize, String)>,
    dropoff_ingestor: Option<(usize, String)>,
    pickup_dispenser: Option<(usize, String)>,
}

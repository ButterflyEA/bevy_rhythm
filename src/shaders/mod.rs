use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::render::render_graph::{RenderGraph, RenderLabel};
use bevy::sprite::{Material2d,Material2dPlugin};
use crate::types::Directions;
use crate::consts::*;


pub mod background;
pub mod target_arrows;
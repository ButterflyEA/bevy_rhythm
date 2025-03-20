use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::{mesh::MeshVertexBufferLayoutRef,
                    render_resource::{AsBindGroup, 
                        RenderPipelineDescriptor, ShaderRef, 
                        SpecializedMeshPipelineError}};
use bevy::sprite::{Material2d, Material2dKey};

pub mod background;
//use background::*;
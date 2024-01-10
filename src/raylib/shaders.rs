

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::collections::HashMap;

use raylib_ffi::enums::ShaderUniformDataType;


//= Enumerations

/// The values for shader variables
pub enum ShaderValueData {
	Float { v: f32 },
	Vector2 { v: super::vectors::Vector2 },
	Vector3 { v: super::vectors::Vector3 },
	Vector4 { v: super::vectors::Vector4 },

	Integer { v: i32 },
	IVector2 { v: [i32;2] },
	IVector3 { v: [i32;3] },
	IVector4 { v: [i32;4] },

	Matrix { v: super::matrixes::Matrix },
	Texture { v: super::textures::Texture },
}


//= Structures

/// Shader type (generic)
#[derive(Clone)]
pub struct Shader {
	pub id:		u32,
	pub locs:	*mut i32,

	pub values: HashMap<String, i32>,
}
impl From<raylib_ffi::Shader> for Shader {
	fn from(value: raylib_ffi::Shader) -> Self {
		Self {
			id: value.id,
			locs: value.locs,
			values: HashMap::new(),
		}
	}
}
impl Into<raylib_ffi::Shader> for Shader {
	fn into(self) -> raylib_ffi::Shader {
		return raylib_ffi::Shader {
			id: self.id,
			locs: self.locs,
		}
	}
}


//= Procedures

impl Shader {
	
	/// Load shader
	pub fn load(vertex: String, fragment: String) -> Self {
		unsafe {
			return Shader::from(raylib_ffi::LoadShader(raylib_ffi::rl_str!(vertex), raylib_ffi::rl_str!(fragment)));
		}
	}

	/// Unload shader
	pub fn unload(&self) {
		unsafe {
			raylib_ffi::UnloadShader(self.clone().into());
		}
	}

	//
	pub fn set_value(&self, valueName: String, value: ShaderValueData) {
		unsafe {
			if self.values.contains_key(&valueName) {
				match value {
					ShaderValueData::Float { v } => {
						raylib_ffi::SetShaderValue(
							self.clone().into(),
							self.values[&valueName],
							[v].as_ptr().cast(),
							ShaderUniformDataType::Float as i32,
						);
					}
					ShaderValueData::Vector2 { v } => {
						let vec: [f32;2] = v.into();
						raylib_ffi::SetShaderValue(
							self.clone().into(),
							self.values[&valueName],
							vec.as_ptr().cast(),
							ShaderUniformDataType::Vec2 as i32,
						);
					}
					ShaderValueData::Vector3 { v } => {
						let vec: [f32;3] = v.into();
						raylib_ffi::SetShaderValue(
							self.clone().into(),
							self.values[&valueName],
							vec.as_ptr().cast(),
							ShaderUniformDataType::Vec3 as i32,
						);
					}
					ShaderValueData::Vector4 { v } => {
						let vec: [f32;4] = v.into();
						raylib_ffi::SetShaderValue(
							self.clone().into(),
							self.values[&valueName],
							vec.as_ptr().cast(),
							ShaderUniformDataType::Vec4 as i32,
						);
					}

					ShaderValueData::Integer { v } => {
						raylib_ffi::SetShaderValue(
							self.clone().into(),
							self.values[&valueName],
							[v].as_ptr().cast(),
							ShaderUniformDataType::Int as i32,
						);
					}
					ShaderValueData::IVector2 { v } => {
						raylib_ffi::SetShaderValue(
							self.clone().into(),
							self.values[&valueName],
							v.as_ptr().cast(),
							ShaderUniformDataType::Ivec2 as i32,
						);
					}
					ShaderValueData::IVector3 { v } => {
						raylib_ffi::SetShaderValue(
							self.clone().into(),
							self.values[&valueName],
							v.as_ptr().cast(),
							ShaderUniformDataType::Ivec3 as i32,
						);
					}
					ShaderValueData::IVector4 { v } => {
						raylib_ffi::SetShaderValue(
							self.clone().into(),
							self.values[&valueName],
							v.as_ptr().cast(),
							ShaderUniformDataType::Ivec4 as i32,
						);
					}
					
					ShaderValueData::Matrix { v } => {
						raylib_ffi::SetShaderValueMatrix(
							self.clone().into(),
							self.values[&valueName],
							v.into(),
						);
					}
					ShaderValueData::Texture { v } => {
						raylib_ffi::SetShaderValueTexture(
							self.clone().into(),
							self.values[&valueName],
							v.into(),
						);
					}
				}
			} else {
				let valueLocation = raylib_ffi::GetShaderLocation(
					self.clone().into(),
					raylib_ffi::rl_str!(valueName),
				);
				self.values.insert(valueName, valueLocation);
				self.set_value(valueName.to_string(), value)
			}
			//self.values.insert(valueName, value)
		}
	}
	
}
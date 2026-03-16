//! VORTEX Biome System - Living visual ecosystems

use std::time::Duration;
use anyhow::Result;

/// Available biome types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiomeType {
    /// Organic, flowing, nature-inspired
    AuraFlow,
    /// Premium glass with sharp geometry
    CrystalEdge,
    /// Dark mode with ambient glow
    MidnightPulse,
    /// Nature particles
    ForestHaze,
    /// Minimal ripple
    ZenStone,
}

impl Default for BiomeType {
    fn default() -> Self {
        Self::AuraFlow
    }
}

/// Biome parameters for shader uniforms
#[derive(Debug, Clone)]
pub struct BiomeParams {
    /// Current biome type
    pub biome_type: BiomeType,
    /// Primary color (RGB, 0-1)
    pub primary_color: [f32; 3],
    /// Secondary color (RGB, 0-1)
    pub secondary_color: [f32; 3],
    /// Accent color (RGB, 0-1)
    pub accent_color: [f32; 3],
    /// Background color (RGB, 0-1)
    pub background_color: [f32; 3],
    /// Animation speed (0-2)
    pub animation_speed: f32,
    /// Glow intensity (0-1)
    pub glow_intensity: f32,
    /// Blur amount (0-1)
    pub blur_amount: f32,
    /// Particle density (0-1)
    pub particle_density: f32,
    /// Transition progress (0-1)
    pub transition_progress: f32,
}

/// Biome system - manages visual themes and transitions
pub struct BiomeSystem {
    /// Current active biome
    pub current_biome: BiomeType,
    /// Current biome parameters
    pub params: BiomeParams,
    /// Target biome for transitions
    target_biome: Option<BiomeType>,
    /// Transition progress (0-1)
    transition_progress: f32,
    /// Animation time accumulator
    time: f32,
}

impl BiomeSystem {
    /// Create new biome system with default biome
    pub fn new() -> Result<Self> {
        let biome_type = BiomeType::default();
        let params = Self::get_biome_params(biome_type);
        
        Ok(Self {
            current_biome: biome_type,
            params,
            target_biome: None,
            transition_progress: 1.0,
            time: 0.0,
        })
    }
    
    /// Get biome parameters for a specific type
    fn get_biome_params(biome: BiomeType) -> BiomeParams {
        match biome {
            BiomeType::AuraFlow => BiomeParams {
                biome_type: biome,
                primary_color: [0.15, 0.08, 0.25],
                secondary_color: [0.25, 0.12, 0.35],
                accent_color: [0.5, 0.25, 0.7],
                background_color: [0.05, 0.03, 0.1],
                animation_speed: 1.0,
                glow_intensity: 0.4,
                blur_amount: 0.8,
                particle_density: 0.3,
                transition_progress: 1.0,
            },
            BiomeType::CrystalEdge => BiomeParams {
                biome_type: biome,
                primary_color: [0.1, 0.12, 0.18],
                secondary_color: [0.15, 0.18, 0.25],
                accent_color: [0.6, 0.7, 0.9],
                background_color: [0.02, 0.03, 0.05],
                animation_speed: 0.5,
                glow_intensity: 0.6,
                blur_amount: 0.9,
                particle_density: 0.1,
                transition_progress: 1.0,
            },
            BiomeType::MidnightPulse => BiomeParams {
                biome_type: biome,
                primary_color: [0.02, 0.02, 0.05],
                secondary_color: [0.08, 0.05, 0.12],
                accent_color: [0.8, 0.2, 0.4],
                background_color: [0.0, 0.0, 0.02],
                animation_speed: 1.5,
                glow_intensity: 0.7,
                blur_amount: 0.6,
                particle_density: 0.2,
                transition_progress: 1.0,
            },
            BiomeType::ForestHaze => BiomeParams {
                biome_type: biome,
                primary_color: [0.05, 0.12, 0.08],
                secondary_color: [0.08, 0.18, 0.1],
                accent_color: [0.3, 0.7, 0.4],
                background_color: [0.02, 0.05, 0.03],
                animation_speed: 0.8,
                glow_intensity: 0.3,
                blur_amount: 0.7,
                particle_density: 0.6,
                transition_progress: 1.0,
            },
            BiomeType::ZenStone => BiomeParams {
                biome_type: biome,
                primary_color: [0.12, 0.12, 0.12],
                secondary_color: [0.18, 0.18, 0.18],
                accent_color: [0.4, 0.4, 0.4],
                background_color: [0.08, 0.08, 0.08],
                animation_speed: 0.3,
                glow_intensity: 0.2,
                blur_amount: 0.95,
                particle_density: 0.0,
                transition_progress: 1.0,
            },
        }
    }
    
    /// Update biome state each frame
    pub fn update(&mut self, delta: Duration) {
        self.time += delta.as_secs_f32();
        
        // Handle transitions
        if let Some(target) = self.target_biome {
            self.transition_progress += delta.as_secs_f32() * 2.0; // 0.5s transition
            
            if self.transition_progress >= 1.0 {
                self.transition_progress = 1.0;
                self.current_biome = target;
                self.target_biome = None;
                self.params = Self::get_biome_params(target);
            }
        }
        
        self.params.transition_progress = self.transition_progress;
    }
    
    /// Switch to a new biome
    pub fn set_biome(&mut self, biome: BiomeType) {
        if biome != self.current_biome && self.target_biome.is_none() {
            self.target_biome = Some(biome);
            self.transition_progress = 0.0;
            tracing::info!("Switching biome to {:?}", biome);
        }
    }
    
    /// Get current biome name as string
    pub fn get_current_biome_name(&self) -> &str {
        match self.current_biome {
            BiomeType::AuraFlow => "AuraFlow",
            BiomeType::CrystalEdge => "CrystalEdge",
            BiomeType::MidnightPulse => "MidnightPulse",
            BiomeType::ForestHaze => "ForestHaze",
            BiomeType::ZenStone => "ZenStone",
        }
    }
    
    /// Get current time for shader
    pub fn get_time(&self) -> f32 {
        self.time
    }
}

impl Default for BiomeSystem {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

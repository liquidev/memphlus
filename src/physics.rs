//! An easy to use wrapper over Rapier.

use glam::Vec2;
use rapier2d::prelude::{
   BroadPhase, CCDSolver, ColliderSet, IntegrationParameters, IslandManager, JointSet, NarrowPhase,
   PhysicsPipeline, RigidBodySet,
};

use crate::common::mint;

pub struct Physics {
   pub gravity: Vec2,
   pub rigid_bodies: RigidBodySet,
   pub colliders: ColliderSet,
   pub parameters: IntegrationParameters,
   pub pipeline: PhysicsPipeline,
   pub island_manager: IslandManager,
   pub broad_phase: BroadPhase,
   pub narrow_phase: NarrowPhase,
   pub joints: JointSet,
   pub ccd_solver: CCDSolver,
}

impl Physics {
   /// Creates a new bundle of physics state, with the specified gravational force vector.
   pub fn new(gravity: Vec2) -> Self {
      Self {
         gravity,
         rigid_bodies: RigidBodySet::new(),
         colliders: ColliderSet::new(),
         parameters: IntegrationParameters {
            dt: (crate::TIMESTEP / 2.0) as f32,
            erp: 1.0,
            ..IntegrationParameters::default()
         },
         pipeline: PhysicsPipeline::new(),
         island_manager: IslandManager::new(),
         broad_phase: BroadPhase::new(),
         narrow_phase: NarrowPhase::new(),
         joints: JointSet::new(),
         ccd_solver: CCDSolver::new(),
      }
   }

   /// Steps the physics state.
   pub fn step(&mut self) {
      // Perform two steps to hopefully make penetrations less obvious.
      for _ in 0..2 {
         self.pipeline.step(
            &mint(self.gravity),
            &self.parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.ccd_solver,
            &(),
            &(),
         )
      }
   }
}

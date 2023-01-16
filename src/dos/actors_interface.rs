/*!
# GMT Finite Element Model client

The module implements the client interface for the [GMT FEM Rust API](https://docs.rs/gmt-fem)

*/

use crate::{
    dos::{DiscreteModalSolver, Get, Set, Solver},
    fem_io,
};
use dos_actors::{
    io::{Data, Read, UniqueIdentifier, Write},
    Size, Update,
};
use std::sync::Arc;

impl<S> Update for DiscreteModalSolver<S>
where
    DiscreteModalSolver<S>: Iterator,
    S: Solver + Default,
{
    fn update(&mut self) {
        log::debug!("update");
        self.next();
    }
}

impl<S, U: UniqueIdentifier<Data = Vec<f64>>> Read<U> for DiscreteModalSolver<S>
where
    Vec<Option<fem_io::Inputs>>: fem_io::FemIo<U>,
    S: Solver + Default,
    U: 'static,
{
    fn read(&mut self, data: Arc<Data<U>>) {
        <DiscreteModalSolver<S> as Set<U>>::set(self, &**data)
    }
}

impl<S, U: UniqueIdentifier<Data = Vec<f64>>> Write<U> for DiscreteModalSolver<S>
where
    Vec<Option<fem_io::Outputs>>: fem_io::FemIo<U>,
    S: Solver + Default,
    U: 'static,
{
    fn write(&mut self) -> Option<Arc<Data<U>>> {
        <DiscreteModalSolver<S> as Get<U>>::get(self).map(|data| Arc::new(Data::new(data)))
    }
}

// # MOUNT CONTROL
use dos_clients_io::{MountEncoders, MountTorques};

impl<S> Get<MountEncoders> for DiscreteModalSolver<S>
where
    S: Solver + Default,
{
    fn get(&self) -> Option<Vec<f64>> {
        let mut encoders = <DiscreteModalSolver<S> as Get<fem_io::OSSAzEncoderAngle>>::get(self)?;
        encoders.extend(
            <DiscreteModalSolver<S> as Get<fem_io::OSSElEncoderAngle>>::get(self)?.as_slice(),
        );
        encoders.extend(
            <DiscreteModalSolver<S> as Get<fem_io::OSSRotEncoderAngle>>::get(self)?.as_slice(),
        );
        Some(encoders)
    }
}
impl<S> Write<MountEncoders> for DiscreteModalSolver<S>
where
    S: Solver + Default,
{
    fn write(&mut self) -> Option<Arc<Data<MountEncoders>>> {
        <DiscreteModalSolver<S> as Get<MountEncoders>>::get(self)
            .map(|data| Arc::new(Data::new(data)))
    }
}
impl<S: Solver + Default> Set<MountTorques> for DiscreteModalSolver<S> {
    fn set(&mut self, u: &[f64]) {
        let (azimuth, others) = u.split_at(12);
        <DiscreteModalSolver<S> as Set<fem_io::OSSAzDriveTorque>>::set(self, azimuth);
        let (elevation, gir) = others.split_at(4);
        <DiscreteModalSolver<S> as Set<fem_io::OSSElDriveTorque>>::set(self, elevation);
        <DiscreteModalSolver<S> as Set<fem_io::OSSRotDriveTorque>>::set(self, gir);
    }
}
impl<S> Read<MountTorques> for DiscreteModalSolver<S>
where
    S: Solver + Default,
{
    fn read(&mut self, data: Arc<Data<MountTorques>>) {
        <DiscreteModalSolver<S> as Set<MountTorques>>::set(self, &data);
    }
}

// # M1 CONTROL
use dos_clients_io::{M1ModeShapes, M1RigidBodyMotions, M2RigidBodyMotions};
//  * M1 mode shape
impl<S> Get<M1ModeShapes> for DiscreteModalSolver<S>
where
    S: Solver + Default,
{
    fn get(&self) -> Option<Vec<f64>> {
        let mut encoders = <DiscreteModalSolver<S> as Get<fem_io::M1Segment1AxialD>>::get(self)?;
        encoders.extend(
            <DiscreteModalSolver<S> as Get<fem_io::M1Segment2AxialD>>::get(self)?.as_slice(),
        );
        encoders.extend(
            <DiscreteModalSolver<S> as Get<fem_io::M1Segment3AxialD>>::get(self)?.as_slice(),
        );
        encoders.extend(
            <DiscreteModalSolver<S> as Get<fem_io::M1Segment4AxialD>>::get(self)?.as_slice(),
        );
        encoders.extend(
            <DiscreteModalSolver<S> as Get<fem_io::M1Segment5AxialD>>::get(self)?.as_slice(),
        );
        encoders.extend(
            <DiscreteModalSolver<S> as Get<fem_io::M1Segment6AxialD>>::get(self)?.as_slice(),
        );
        encoders.extend(
            <DiscreteModalSolver<S> as Get<fem_io::M1Segment7AxialD>>::get(self)?.as_slice(),
        );
        Some(encoders)
    }
}
impl<S> Write<M1ModeShapes> for DiscreteModalSolver<S>
where
    S: Solver + Default,
{
    fn write(&mut self) -> Option<Arc<Data<M1ModeShapes>>> {
        <DiscreteModalSolver<S> as Get<M1ModeShapes>>::get(self)
            .map(|data| Arc::new(Data::new(data)))
    }
}
//  * M1 rigid body motions
impl<S> Size<M1RigidBodyMotions> for DiscreteModalSolver<S>
where
    DiscreteModalSolver<S>: Iterator,
    S: Solver + Default,
{
    fn len(&self) -> usize {
        42
    }
}
impl<S> Write<M1RigidBodyMotions> for DiscreteModalSolver<S>
where
    S: Solver + Default,
{
    fn write(&mut self) -> Option<Arc<Data<M1RigidBodyMotions>>> {
        <DiscreteModalSolver<S> as Get<fem_io::OSSM1Lcl>>::get(self)
            .map(|data| Arc::new(Data::new(data)))
    }
}
//  * M2 rigid body motions
impl<S> Size<M2RigidBodyMotions> for DiscreteModalSolver<S>
where
    DiscreteModalSolver<S>: Iterator,
    S: Solver + Default,
{
    fn len(&self) -> usize {
        42
    }
}
#[cfg(not(feature = "mcm2lcl"))]
impl<S> Write<M2RigidBodyMotions> for DiscreteModalSolver<S>
where
    S: Solver + Default,
{
    fn write(&mut self) -> Option<Arc<Data<M2RigidBodyMotions>>> {
        <DiscreteModalSolver<S> as Get<fem_io::MCM2Lcl6D>>::get(self)
            .map(|data| Arc::new(Data::new(data)))
    }
}
#[cfg(feature = "mcm2lcl")]
impl<S> Write<M2RigidBodyMotions> for DiscreteModalSolver<S>
where
    S: Solver + Default,
{
    fn write(&mut self) -> Option<Arc<Data<M2RigidBodyMotions>>> {
        <DiscreteModalSolver<S> as Get<fem_io::MCM2Lcl>>::get(self)
            .map(|data| Arc::new(Data::new(data)))
    }
}
// # M2 CONTROL
use dos_clients_io::{M2PositionerForces, M2PositionerNodes};
// ## M2 positioner
//  * forces
impl<S> Read<M2PositionerForces> for DiscreteModalSolver<S>
where
    S: Solver + Default,
{
    fn read(&mut self, data: Arc<Data<M2PositionerForces>>) {
        <DiscreteModalSolver<S> as Set<fem_io::MCM2SmHexF>>::set(self, &data)
    }
}
// * nodes
impl<S> Write<M2PositionerNodes> for DiscreteModalSolver<S>
where
    S: Solver + Default,
{
    fn write(&mut self) -> Option<Arc<Data<M2PositionerNodes>>> {
        <DiscreteModalSolver<S> as Get<fem_io::MCM2SmHexD>>::get(self)
            .map(|data| Arc::new(Data::new(data)))
    }
}
#[cfg(feature = "fsm")]
pub mod fsm {
    use super::*;
    use dos_clients_io::{M2FSMPiezoForces, M2FSMPiezoNodes};
    // ## M2 FSM PZT
    // *  forces
    impl<S> Read<M2FSMPiezoForces> for DiscreteModalSolver<S>
    where
        S: Solver + Default,
    {
        fn read(&mut self, data: Arc<Data<M2FSMPiezoForces>>) {
            <DiscreteModalSolver<S> as Set<fem_io::MCM2PZTF>>::set(self, &data)
        }
    }
    // * nodes
    impl<S> Write<M2FSMPiezoNodes> for DiscreteModalSolver<S>
    where
        S: Solver + Default,
    {
        fn write(&mut self) -> Option<Arc<Data<M2FSMPiezoNodes>>> {
            <DiscreteModalSolver<S> as Get<fem_io::MCM2PZTD>>::get(self)
                .map(|data| Arc::new(Data::new(data)))
        }
    }
}
#[cfg(feature = "asm")]
pub mod asm {
    use super::*;
    use dos_clients_io::{
        M2ASMColdPlateForces, M2ASMFaceSheetForces, M2ASMFaceSheetNodes, M2ASMRigidBodyForces,
        M2ASMRigidBodyNodes,
    };
    // ## M2 ASM
    // * rigid body
    //  * forces
    impl<S> Read<M2ASMRigidBodyForces> for DiscreteModalSolver<S>
    where
        S: Solver + Default,
    {
        fn read(&mut self, data: Arc<Data<M2ASMRigidBodyForces>>) {
            <DiscreteModalSolver<S> as Set<fem_io::MCM2RB6F>>::set(self, &data)
        }
    }
    // * nodes
    impl<S> Write<M2ASMRigidBodyNodes> for DiscreteModalSolver<S>
    where
        S: Solver + Default,
    {
        fn write(&mut self) -> Option<Arc<Data<M2ASMRigidBodyNodes>>> {
            <DiscreteModalSolver<S> as Get<fem_io::MCM2RB6D>>::get(self)
                .map(|data| Arc::new(Data::new(data)))
        }
    }
    // * cold plate
    //  * forces
    impl<S> Read<M2ASMColdPlateForces> for DiscreteModalSolver<S>
    where
        S: Solver + Default,
    {
        fn read(&mut self, data: Arc<Data<M2ASMColdPlateForces>>) {
            <DiscreteModalSolver<S> as Set<fem_io::MCM2CP6F>>::set(self, &data)
        }
    }
    // * face sheet
    //  * forces
    impl<S> Read<M2ASMFaceSheetForces> for DiscreteModalSolver<S>
    where
        S: Solver + Default,
    {
        fn read(&mut self, data: Arc<Data<M2ASMFaceSheetForces>>) {
            <DiscreteModalSolver<S> as Set<fem_io::MCM2Lcl6F>>::set(self, &data)
        }
    }
    // * nodes
    impl<S> Write<M2ASMFaceSheetNodes> for DiscreteModalSolver<S>
    where
        S: Solver + Default,
    {
        fn write(&mut self) -> Option<Arc<Data<M2ASMFaceSheetNodes>>> {
            <DiscreteModalSolver<S> as Get<fem_io::MCM2Lcl6D>>::get(self)
                .map(|data| Arc::new(Data::new(data)))
        }
    }
}

#[doc(hidden)]
#[cfg(feature = "cfd2022")]
pub mod cfd {
    use super::{fem_io, Arc, Data, DiscreteModalSolver, Read, Set, Solver};
    // # CFD
    // * mount
    use dos_clients_io::{CFDM1WindLoads, CFDMountWindLoads};
    impl<S> Read<CFDMountWindLoads> for DiscreteModalSolver<S>
    where
        S: Solver + Default,
    {
        fn read(&mut self, data: Arc<Data<CFDMountWindLoads>>) {
            <DiscreteModalSolver<S> as Set<fem_io::CFD2021106F>>::set(self, &data)
        }
    }
    // * M1
    impl<S> Read<CFDM1WindLoads> for DiscreteModalSolver<S>
    where
        S: Solver + Default,
    {
        fn read(&mut self, data: Arc<Data<CFDM1WindLoads>>) {
            <DiscreteModalSolver<S> as Set<fem_io::OSSM1Lcl6F>>::set(self, &data)
        }
    }
}
// * M2
#[cfg(feature = "asm")]
impl<S> Read<dos_clients_io::CFDM2WindLoads> for DiscreteModalSolver<S>
where
    S: Solver + Default,
{
    fn read(&mut self, data: Arc<Data<dos_clients_io::CFDM2WindLoads>>) {
        <DiscreteModalSolver<S> as Set<fem_io::MCM2Lcl6F>>::set(self, &data)
    }
}
#[cfg(feature = "fsm")]
impl<S> Read<dos_clients_io::CFDM2WindLoads> for DiscreteModalSolver<S>
where
    S: Solver + Default,
{
    fn read(&mut self, data: Arc<Data<dos_clients_io::CFDM2WindLoads>>) {
        <DiscreteModalSolver<S> as Set<fem_io::MCM2LclForce6F>>::set(self, &data)
    }
}

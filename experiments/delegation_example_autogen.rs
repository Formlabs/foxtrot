// This file was autogenerated, do not modify
use crate::id::{dynamic_cast, Id};
use ambassador::{delegatable_trait, Delegate};
use std::any::{Any, TypeId};
use std::collections::HashMap;

// Types for Animal entity:

#[derive(Clone, Debug)]

pub struct AnimalData {
    pub name: String,
}

#[delegatable_trait]
pub trait AnimalTrait {
    fn name(&self) -> &String;
}

impl AnimalTrait for AnimalData {
    fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Clone, Debug, Delegate)]
#[delegate(AnimalTrait)]
pub enum Animal {
    Mammal(Mammal),
}

// Types for Mammal entity:

#[derive(Clone, Debug, Delegate)]
#[delegate(AnimalTrait, target = "parent_animal")]
pub struct MammalData {
    pub parent_animal: AnimalData,
    pub hair_type: String,
}

#[delegatable_trait]
pub trait MammalTrait {
    fn hair_type(&self) -> &String;
}

impl MammalTrait for MammalData {
    fn hair_type(&self) -> &String {
        &self.hair_type
    }
}

#[derive(Clone, Debug, Delegate)]
#[delegate(AnimalTrait)]
#[delegate(MammalTrait)]
pub enum Mammal {
    Dog(Dog),
    Cat(Cat),
}

// Types for ExtraCategory entity:

#[derive(Clone, Debug)]

pub struct ExtraCategoryData {
    pub extra: usize,
}

#[delegatable_trait]
pub trait ExtraCategoryTrait {
    fn extra(&self) -> &usize;
}

impl ExtraCategoryTrait for ExtraCategoryData {
    fn extra(&self) -> &usize {
        &self.extra
    }
}

#[derive(Clone, Debug, Delegate)]
#[delegate(ExtraCategoryTrait)]
pub enum ExtraCategory {
    Dog(Dog),
}

// Types for Dog entity:

#[derive(Clone, Debug, Delegate)]
#[delegate(MammalTrait, target = "parent_mammal")]
#[delegate(AnimalTrait, target = "parent_mammal")]
#[delegate(ExtraCategoryTrait, target = "parent_extracategory")]
pub struct Dog {
    pub parent_mammal: MammalData,
    pub parent_extracategory: ExtraCategoryData,
    pub dog_breed: String,
}

#[delegatable_trait]
pub trait DogTrait {
    fn dog_breed(&self) -> &String;
}

impl DogTrait for Dog {
    fn dog_breed(&self) -> &String {
        &self.dog_breed
    }
}

// Types for Cat entity:

#[derive(Clone, Debug, Delegate)]
#[delegate(MammalTrait, target = "parent_mammal")]
#[delegate(AnimalTrait, target = "parent_mammal")]
pub struct Cat {
    pub parent_mammal: MammalData,
    pub cat_breed: String,
}

#[delegatable_trait]
pub trait CatTrait {
    fn cat_breed(&self) -> &String;
}

impl CatTrait for Cat {
    fn cat_breed(&self) -> &String {
        &self.cat_breed
    }
}

// Types for VetRecord entity:

#[derive(Clone, Debug)]

pub struct VetRecord {
    pub diagnosis: String,
    pub patient: Id<Mammal>,
}

#[delegatable_trait]
pub trait VetRecordTrait {
    fn diagnosis(&self) -> &String;
    fn patient(&self) -> &Id<Mammal>;
}

impl VetRecordTrait for VetRecord {
    fn diagnosis(&self) -> &String {
        &self.diagnosis
    }

    fn patient(&self) -> &Id<Mammal> {
        &self.patient
    }
}

pub(crate) fn lookup_autogen<T>(id: &Id<T>, storage: &HashMap<usize, Box<dyn Any>>) -> Option<T>
where
    T: Any + Clone,
{
    let dynamic_entity: &Box<dyn Any> = storage.get(&id.raw())?;

    let entity_type_id = (&**dynamic_entity).type_id();
    let requested_type_id = TypeId::of::<T>();

    if entity_type_id == TypeId::of::<Dog>() {
        let static_entity = dynamic_entity
            .downcast_ref::<Dog>()
            .expect("downcasting failed");

        if requested_type_id == TypeId::of::<Dog>() {
            Some(dynamic_cast::<Dog, T>(static_entity).to_owned())
        } else if requested_type_id == TypeId::of::<Animal>() {
            let static_entity = Animal::Mammal(Mammal::Dog(static_entity.to_owned()));
            Some(dynamic_cast::<Animal, T>(&static_entity).to_owned())
        } else if requested_type_id == TypeId::of::<Mammal>() {
            let static_entity = Mammal::Dog(static_entity.to_owned());
            Some(dynamic_cast::<Mammal, T>(&static_entity).to_owned())
        } else if requested_type_id == TypeId::of::<ExtraCategory>() {
            let static_entity = ExtraCategory::Dog(static_entity.to_owned());
            Some(dynamic_cast::<ExtraCategory, T>(&static_entity).to_owned())
        } else {
            None
        }
    } else if entity_type_id == TypeId::of::<Cat>() {
        let static_entity = dynamic_entity
            .downcast_ref::<Cat>()
            .expect("downcasting failed");

        if requested_type_id == TypeId::of::<Cat>() {
            Some(dynamic_cast::<Cat, T>(static_entity).to_owned())
        } else if requested_type_id == TypeId::of::<Animal>() {
            let static_entity = Animal::Mammal(Mammal::Cat(static_entity.to_owned()));
            Some(dynamic_cast::<Animal, T>(&static_entity).to_owned())
        } else if requested_type_id == TypeId::of::<Mammal>() {
            let static_entity = Mammal::Cat(static_entity.to_owned());
            Some(dynamic_cast::<Mammal, T>(&static_entity).to_owned())
        } else {
            None
        }
    } else if entity_type_id == TypeId::of::<VetRecord>() {
        let static_entity = dynamic_entity
            .downcast_ref::<VetRecord>()
            .expect("downcasting failed");

        if requested_type_id == TypeId::of::<VetRecord>() {
            Some(dynamic_cast::<VetRecord, T>(static_entity).to_owned())
        } else {
            None
        }
    } else {
        None
    }
}

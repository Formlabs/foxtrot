use ambassador::{delegatable_trait, Delegate};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct Id<T>(usize, PhantomData<*const T>);
impl<T> Id<T> {
    pub const fn new(u: usize) -> Self {
        Self(u, PhantomData)
    }

    fn raw(&self) -> usize {
        self.0
    }
}

////////////////////////////////////////////////////////////
// Types and traits that will eventually be auto-generated:

#[derive(Clone, Debug)]
pub struct AnimalData {
    name: String,
}
#[delegatable_trait]
trait AnimalTrait {
    fn name(&self) -> &str;
}
impl AnimalTrait for AnimalData {
    fn name(&self) -> &str {
        &self.name
    }
}
#[derive(Clone, Debug, Delegate)]
#[delegate(AnimalTrait)]
pub enum Animal {
    Mammal(Mammal),
}

#[derive(Clone, Debug, Delegate)]
#[delegate(AnimalTrait, target = "parent")]
pub struct MammalData {
    parent: AnimalData,
    hair_type: String,
}
#[delegatable_trait]
trait MammalTrait {
    fn hair_type(&self) -> &str;
}
impl MammalTrait for MammalData {
    fn hair_type(&self) -> &str {
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

#[derive(Clone, Debug, Delegate)]
#[delegate(AnimalTrait, target = "parent")]
#[delegate(MammalTrait, target = "parent")]
pub struct Cat {
    parent: MammalData,
    cat_breed: String,
}
#[delegatable_trait]
trait CatTrait {
    fn cat_breed(&self) -> &str;
}
impl CatTrait for Cat {
    fn cat_breed(&self) -> &str {
        &self.cat_breed
    }
}

// For demonstrating multiple inheritance
#[derive(Clone, Debug)]
pub struct ExtraCategoryData {
    extra: usize,
}
#[delegatable_trait]
trait ExtraCategoryTrait {
    fn extra(&self) -> usize;
}
impl ExtraCategoryTrait for ExtraCategoryData {
    fn extra(&self) -> usize {
        self.extra
    }
}
#[derive(Clone, Debug, Delegate)]
#[delegate(ExtraCategoryTrait)]
pub enum ExtraCategory {
    Dog(Dog),
}

#[derive(Clone, Debug, Delegate)]
#[delegate(AnimalTrait, target = "parent_mammal")]
#[delegate(MammalTrait, target = "parent_mammal")]
#[delegate(ExtraCategoryTrait, target = "parent_extra")]
pub struct Dog {
    parent_mammal: MammalData,
    parent_extra: ExtraCategoryData,
    dog_breed: String,
}
#[delegatable_trait]
trait DogTrait {
    fn dog_breed(&self) -> &str;
}
impl DogTrait for Dog {
    fn dog_breed(&self) -> &str {
        &self.dog_breed
    }
}

#[derive(Clone, Debug)]
pub struct VetRecord {
    diagnosis: String,
    patient: Id<Mammal>,
}
#[delegatable_trait]
trait VetRecordTrait {
    fn diagnosis(&self) -> &str;
    fn patient(&self) -> Id<Mammal>;
}
impl VetRecordTrait for VetRecord {
    fn diagnosis(&self) -> &str {
        &self.diagnosis
    }
    fn patient(&self) -> Id<Mammal> {
        self.patient.clone()
    }
}

//////////////////////////////////////////////////////

fn dynamic_cast<T, U>(t: &T) -> &U
where
    T: Any,
    U: Any,
{
    let any: &dyn Any = t;
    any.downcast_ref::<U>()
        .expect("dynamic casting failed, T and U are not the same type!")
}

// Use the `Any` trait to store any of our types in the HashMap, instead of creating a big global enum for all of them.
#[derive(Debug)]
pub struct Storage(HashMap<usize, Box<dyn Any>>);

impl Storage {
    pub fn new() -> Self {
        Storage(HashMap::new())
    }

    pub fn insert<T>(&mut self, raw_id: usize, node: T)
    where
        T: Any,
    {
        self.0.insert(raw_id, Box::new(node));
    }

    // TODO is it possible to avoid cloning so much? Maybe not, because this sometimes needs to
    // construct and return an enum, so it can't return a reference to T. And the enums themselves
    // can't contain non-static references because of constraints on `Any`.
    pub fn get<T>(&self, id: Id<T>) -> Option<T>
    where
        T: Any + Clone,
    {
        let node: &Box<dyn Any> = self.0.get(&id.raw())?;

        let node_type = (&**node).type_id();
        let requested_type = TypeId::of::<T>();

        // TODO autogenerate these cases based on the STEP type hierarchy
        if node_type == TypeId::of::<Dog>() {
            let dog: &Dog = node.downcast_ref::<Dog>().expect("downcasting failed");
            if requested_type == TypeId::of::<Dog>() {
                // We know that T is Dog because we compared the TypeIds, but the type checker
                // doesn't know that, so we need to dynamically cast it to T.
                Some(dynamic_cast::<Dog, T>(dog).to_owned())
            } else if requested_type == TypeId::of::<Mammal>() {
                let mammal = Mammal::Dog(dog.to_owned());
                Some(dynamic_cast::<Mammal, T>(&mammal).to_owned())
            } else if requested_type == TypeId::of::<Animal>() {
                let animal = Animal::Mammal(Mammal::Dog(dog.to_owned()));
                Some(dynamic_cast::<Animal, T>(&animal).to_owned())
            } else if requested_type == TypeId::of::<ExtraCategory>() {
                let extra = ExtraCategory::Dog(dog.to_owned());
                Some(dynamic_cast::<ExtraCategory, T>(&extra).to_owned())
            } else {
                println!("Error: Id's type is incompatible with Dog");
                None
            }
        } else if node_type == TypeId::of::<Cat>() {
            let cat: &Cat = node.downcast_ref::<Cat>().expect("downcasting failed");
            if requested_type == TypeId::of::<Cat>() {
                Some(dynamic_cast::<Cat, T>(cat).to_owned())
            } else if requested_type == TypeId::of::<Mammal>() {
                let mammal = Mammal::Cat(cat.to_owned());
                Some(dynamic_cast::<Mammal, T>(&mammal).to_owned())
            } else if requested_type == TypeId::of::<Animal>() {
                let animal = Animal::Mammal(Mammal::Cat(cat.to_owned()));
                Some(dynamic_cast::<Animal, T>(&animal).to_owned())
            } else {
                println!("Error: Id's type is incompatible with Cat");
                None
            }
        } else if node_type == TypeId::of::<VetRecord>() {
            let record: &VetRecord = node
                .downcast_ref::<VetRecord>()
                .expect("downcasting failed");
            if requested_type == TypeId::of::<VetRecord>() {
                Some(dynamic_cast::<VetRecord, T>(record).to_owned())
            } else {
                println!("Error: Id's type is incompatible with VetRecord");
                None
            }
        } else {
            panic!("unknown type")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delegation() {
        let dog = Dog {
            parent_mammal: MammalData {
                parent: AnimalData {
                    name: "Fido".to_string(),
                },
                hair_type: "curly".to_string(),
            },
            parent_extra: ExtraCategoryData { extra: 42 },
            dog_breed: "poodle".to_string(),
        };
        assert_eq!(dog.name(), "Fido");
        assert_eq!(dog.hair_type(), "curly");
        assert_eq!(dog.dog_breed(), "poodle");
        assert_eq!(dog.extra(), 42);

        let mammal = Mammal::Dog(dog);
        assert_eq!(mammal.name(), "Fido");
        assert_eq!(mammal.hair_type(), "curly");
    }

    #[test]
    fn test_storage() {
        let mut storage = Storage::new();
        storage.insert(
            0,
            Dog {
                parent_mammal: MammalData {
                    parent: AnimalData {
                        name: "Fido".to_string(),
                    },
                    hair_type: "curly".to_string(),
                },
                parent_extra: ExtraCategoryData { extra: 42 },
                dog_breed: "poodle".to_string(),
            },
        );

        let dog_id: Id<Dog> = Id::new(0);
        let dog: Dog = storage.get(dog_id).expect("couldn't get as dog");
        assert_eq!(dog.name(), "Fido");

        let mammal_id: Id<Mammal> = Id::new(0);
        let mammal: Mammal = storage.get(mammal_id).expect("couldn't get as mammal");
        assert_eq!(mammal.name(), "Fido");

        let animal_id: Id<Animal> = Id::new(0);
        let animal: Animal = storage.get(animal_id).expect("couldn't get as animal");
        assert_eq!(animal.name(), "Fido");

        let extra_category_id: Id<ExtraCategory> = Id::new(0);
        let extra_category: ExtraCategory = storage
            .get(extra_category_id)
            .expect("couldn't get as ExtraCategory");
        assert_eq!(extra_category.extra(), 42);

        let bad_type_id: Id<Cat> = Id::new(0);
        assert!(storage.get(bad_type_id).is_none());

        let bad_index_id: Id<Dog> = Id::new(99);
        assert!(storage.get(bad_index_id).is_none());
    }

    #[test]
    fn test_nested_ids() {
        let mut storage = Storage::new();
        storage.insert(
            0,
            Dog {
                parent_mammal: MammalData {
                    parent: AnimalData {
                        name: "Fido".to_string(),
                    },
                    hair_type: "curly".to_string(),
                },
                parent_extra: ExtraCategoryData { extra: 42 },
                dog_breed: "poodle".to_string(),
            },
        );
        storage.insert(
            1,
            VetRecord {
                patient: Id::new(0),
                diagnosis: "too cute".to_string(),
            },
        );

        let vet_record_id: Id<VetRecord> = Id::new(1);
        let vet_record = storage.get(vet_record_id).expect("couldn't get vet record");
        let patient: Mammal = storage
            .get(vet_record.patient())
            .expect("couldn't get patient");

        assert_eq!(patient.name(), "Fido");
        assert_eq!(vet_record.diagnosis(), "too cute");
    }
}

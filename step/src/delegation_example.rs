use std::any::Any;
use std::collections::HashMap;

use crate::delegation_example_autogen::*;
use crate::id::Id;

// Use the `Any` trait to store any of our types in the HashMap, instead of creating a big global enum for all of them.
#[derive(Debug)]
pub struct Storage(HashMap<usize, Box<dyn Any>>);

impl Storage {
    pub fn new() -> Self {
        Storage(HashMap::new())
    }

    pub fn insert<T>(&mut self, raw_id: usize, entity: T)
    where
        T: Any,
    {
        self.0.insert(raw_id, Box::new(entity));
    }

    pub fn get<T>(&self, id: &Id<T>) -> Option<T>
    where
        T: Any + Clone,
    {
        lookup_autogen(id, &self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delegation() {
        let dog = Dog {
            parent_mammal: MammalData {
                parent_animal: AnimalData {
                    name: "Fido".to_string(),
                },
                hair_type: "curly".to_string(),
            },
            parent_extracategory: ExtraCategoryData { extra: 42 },
            dog_breed: "poodle".to_string(),
        };
        assert_eq!(dog.name(), "Fido");
        assert_eq!(dog.hair_type(), "curly");
        assert_eq!(dog.dog_breed(), "poodle");
        assert_eq!(*dog.extra(), 42);

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
                    parent_animal: AnimalData {
                        name: "Fido".to_string(),
                    },
                    hair_type: "curly".to_string(),
                },
                parent_extracategory: ExtraCategoryData { extra: 42 },
                dog_breed: "poodle".to_string(),
            },
        );

        let dog_id: Id<Dog> = Id::new(0);
        let dog: Dog = storage.get(&dog_id).expect("couldn't get as dog");
        assert_eq!(dog.name(), "Fido");

        let mammal_id: Id<Mammal> = Id::new(0);
        let mammal: Mammal = storage.get(&mammal_id).expect("couldn't get as mammal");
        assert_eq!(mammal.name(), "Fido");

        let animal_id: Id<Animal> = Id::new(0);
        let animal: Animal = storage.get(&animal_id).expect("couldn't get as animal");
        assert_eq!(animal.name(), "Fido");

        let extra_category_id: Id<ExtraCategory> = Id::new(0);
        let extra_category: ExtraCategory = storage
            .get(&extra_category_id)
            .expect("couldn't get as ExtraCategory");
        assert_eq!(*extra_category.extra(), 42);

        let bad_type_id: Id<Cat> = Id::new(0);
        assert!(storage.get(&bad_type_id).is_none());

        let bad_index_id: Id<Dog> = Id::new(99);
        assert!(storage.get(&bad_index_id).is_none());
    }

    #[test]
    fn test_nested_ids() {
        let mut storage = Storage::new();
        storage.insert(
            0,
            Dog {
                parent_mammal: MammalData {
                    parent_animal: AnimalData {
                        name: "Fido".to_string(),
                    },
                    hair_type: "curly".to_string(),
                },
                parent_extracategory: ExtraCategoryData { extra: 42 },
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
        let vet_record = storage
            .get(&vet_record_id)
            .expect("couldn't get vet record");
        let patient: Mammal = storage
            .get(vet_record.patient())
            .expect("couldn't get patient");

        assert_eq!(patient.name(), "Fido");
        assert_eq!(vet_record.diagnosis(), "too cute");
    }
}

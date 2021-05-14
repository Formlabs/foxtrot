use ambassador::{delegatable_trait, Delegate};

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

#[derive(Delegate)]
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

#[derive(Delegate)]
#[delegate(AnimalTrait, target = "parent")]
#[delegate(MammalTrait, target = "parent")]
pub enum Mammal {
    Dog(Dog),
    Cat(Cat),
}

#[derive(Delegate)]
#[delegate(AnimalTrait, target = "parent")]
#[delegate(MammalTrait, target = "parent")]
pub struct Dog {
    parent: MammalData,
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

#[derive(Delegate)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delegation() {
        let dog = Dog {
            parent: MammalData {
                parent: AnimalData {
                    name: "Fido".to_string(),
                },
                hair_type: "curly".to_string(),
            },
            dog_breed: "poodle".to_string(),
        };
        assert_eq!(dog.name(), "Fido");
        assert_eq!(dog.hair_type(), "curly");
        assert_eq!(dog.dog_breed(), "poodle");

        let mammal = Mammal::Dog(dog);
        assert_eq!(mammal.name(), "Fido");
        assert_eq!(mammal.hair_type(), "curly");
    }
}

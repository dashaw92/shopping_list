/// Defines the units of measure used in the program, and also how to convert from one to another.

//Maybe this should be split into two sub-enums?
//"Precise" and "Imprecise"- Precise would be measured units such as cups, oz, etc
//Imprecise would be for "pinches" and "wholes"
//Would make it possible to seal up the API against edge cases.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, PartialOrd, Serialize, Deserialize)]
pub enum Unit {
    //Used for "and a pinch of salt" type ingredients, not enough for even a tsp.
    Pinch,
    //Basic culinary units
    Teaspoons,
    Tablespoons,
    Ounces,
    Cups,
    //A whole pepper, 2 potatoes, 1 and a half lemons, etc.
    Whole,
}

impl Unit {
    //Returns a tuple of multiplier + next biggest to actually convert to it
    fn next_biggest(&self) -> (f32, Unit) {
        use Unit::*;
        match *self {
            Cups => (1.0, Cups),
            Ounces => (0.125, Cups), //8 oz to a cup
            Tablespoons => (0.5, Ounces), //2 tbsp to an oz
            Teaspoons => (0.33, Tablespoons), //3 tsp to a tbsp
            Pinch => (0.25, Teaspoons), //4 "pinches" to a tsp
            Whole => (1.0, Whole),
        }
    }

    fn next_smallest(&self) -> (f32, Unit) {
        use Unit::*;
        match *self {
            Cups => (8.0, Ounces),
            Ounces => (2.0, Tablespoons),
            Tablespoons => (3.0, Teaspoons),
            Teaspoons => (4.0, Pinch),
            Pinch => (1.0, Pinch),
            Whole => (1.0, Whole),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Measure {
    pub(crate) quantity: f32,
    pub(crate) unit: Unit,
}

impl Measure {
    pub fn new(unit: Unit) -> Measure {
        Self {
            quantity: 0.0,
            unit
        }
    }
 
    pub fn convert_to(&self, unit: Unit) -> Measure {
        if self.unit == unit || self.unit == Unit::Whole || unit == Unit::Whole {
            return self.clone();
        }

        let mut qty = self.quantity;
        let mut current = self.unit.clone();

        let conversion_fn = if current < unit {
            Unit::next_biggest
        } else {
            Unit::next_smallest
        };

        while current != unit {
            let (multiplier, next_unit) = conversion_fn(&current);
            qty *= multiplier;
            current = next_unit;
        }

        Measure {
            quantity: qty,
            unit: current,
        }
    }
}

use std::ops::Add;

use serde::{Serialize, Deserialize};

impl Add for &Measure {
    type Output = Measure;

    fn add(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit || self.unit == Unit::Whole || rhs.unit == Unit::Whole {
            return Measure {
                quantity: self.quantity + rhs.quantity,
                unit: self.unit.clone(),
            };
        }

        let converted = rhs.convert_to(self.unit.clone());
        Measure {
            quantity: self.quantity + converted.quantity,
            unit: self.unit.clone(),
        }
    }
}
// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use cmn::{
    cmn_constants,
    constants::{Constant, ConstantValue},
};
use mini_functions::common::*;

fn main() {
    // Create a Constants instance
    let c = Constants::new();

    // Retrieve the list of constants
    let constants = c.constants();

    // Serialize the constants for readability
    let serialized = serde_json::to_string_pretty(&constants).unwrap();

    // Print the serialized constants to the console
    println!("🦀 Constants: ✅ {serialized}");

    /// The JSON string to deserialize
    static JSON: &str = r#"[{"name":"EULER","value":"2.718281828459045"},{"name":"PI","value":"3.141592653589793"},{"name":"TAU","value":"6.283185307179586"},{"name":"SQRT2","value":"1.4142135623730951"},{"name":"SQRT1_2","value":"0.7071067811865476"},{"name":"LN2","value":"0.6931471805599453"},{"name":"LN10","value":"2.302585092994046"},{"name":"LOG2E","value":"1.4426950408889634"},{"name":"LOG10E","value":"0.4342944819032518"},{"name":"PHI","value":"1.618033988749895"},{"name":"GOLDEN_RATIO","value":"1.618033988749895"},{"name":"INFINITY","value":"Infinity"},{"name":"NEG_INFINITY","value":"-Infinity"},{"name":"NAN","value":"NaN"}]"#;

    // Deserialize the constants from the JSON string
    let deserialized: Vec<Constant> =
        serde_json::from_str(JSON).unwrap();

    // Print the deserialized constants
    println!("🦀 Deserialized: ✅ {deserialized:?}");

    // Convert a constant value to ConstantValue
    let pi = ConstantValue::Float(std::f64::consts::PI);
    println!("🦀 ConstantValue of PI: ✅ {pi:?}");

    // Retrieve a constant by name and print it (in this case, EULER)
    let euler_constant = c.constant("EULER");
    println!("🦀 ConstantValue of EULER: ✅ {euler_constant:?}");

    // Retrieve a constant by name and print it (in this case, PI)
    cmn_constants! {
        AVOGADRO = cmn::constants::AVOGADRO,
        BOLTZMANN = cmn::constants::BOLTZMANN,
        EULER = cmn::constants::EULER,
        GAMMA = cmn::constants::GAMMA,
        PHI = cmn::constants::PHI,
        PI = cmn::constants::PI,
        PLANCK = cmn::constants::PLANCK,
        SILVER_RATIO = cmn::constants::SILVER_RATIO,
        SQRT2 = cmn::constants::SQRT2,
        SQRT3 = cmn::constants::SQRT3,
        SQRT5 = cmn::constants::SQRT5,
        TAU = cmn::constants::TAU
    }
    println!("🦀 Using cmn_constants! macro:");
    println!("- Avogadro's constant:     ✅ {}", AVOGADRO);
    println!("- Boltzmann's constant:    ✅ {}", BOLTZMANN);
    println!("- Euler's constant:        ✅ {}", EULER);
    println!("- Gamma's constant:        ✅ {}", GAMMA);
    println!("- Phi's constant:          ✅ {}", PHI);
    println!("- Pi's constant:           ✅ {}", PI);
    println!("- Planck's constant:       ✅ {}", PLANCK);
    println!("- Silver ratio's constant: ✅ {}", SILVER_RATIO);
    println!("- Sqrt2's constant:        ✅ {}", SQRT2);
    println!("- Sqrt3's constant:        ✅ {}", SQRT3);
    println!("- Sqrt5's constant:        ✅ {}", SQRT5);
    println!("- Tau's constant:          ✅ {}", TAU);
}

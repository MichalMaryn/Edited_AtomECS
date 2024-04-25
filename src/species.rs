//! Predefined species that can be used in AtomECS.

/// # Arguments:
/// * `transition_name`: name of the generated struct.
/// * `frequency`: frequency of the laser cooling transition, in Hz.
/// * `linewidth`: linewidth of the laser cooling transition, in Hz.
/// * `saturation_intensity`: Saturation intensity, in units of W/m^2. W/m^2 = 10*mW/cm^2
/// * `mup`: shift of the sigma+ transition in magnetic field.
/// * `mum`: shift of the sigma- transition in magnetic field.
/// * `muz`: shift of the pi transition in magnetic field.

use crate::constant::BOHRMAG;
use crate::{transition, species};

species!(Strontium88, Strontium88_461, 88);
transition!(Strontium88_461, 
    650_759_219_088_937.0, 
    32e6, 
    430.0, 
    BOHRMAG, 
    -BOHRMAG, 
    0.0
);

transition!(
    Strontium88_689,
    434_829_121_311_000.0,
    7.4e3,
    0.0295,
    BOHRMAG,
    -BOHRMAG,
    0.0
); 

species!(Rubidium87, Rubidium87_780D2, 87);
transition!(Rubidium87_780D2,
    384_228_115_202_521.0, 
    6.065e6, 
    16.69, 
    BOHRMAG, 
    -BOHRMAG, 
    0.0
); //[Steck, 87 D2]

species!(Potassium39, Potassium39_767D2, 39);
transition!(
    Potassium39_767D2, // [Tiecke 2019]
    391_016_170_030_000.0,
    6.035e6,
    17.5,
    BOHRMAG,
    -BOHRMAG,
    0.0
); 

species!(Potassium40, Potassium40_767D2, 40);
transition!(
    Potassium40_767D2, // [Tiecke 2019]
    391_016_296_050_000.0, // 4 2S1/2 - 4 2P3/2 transition
    6.035e6, // Natural Line Width
    17.5, // Saturation intensity W/m^2 = 10*mW/cm^2
    BOHRMAG,
    -BOHRMAG,
    0.0
); 

species!(Potassium41, Potassium41_767D2, 41);
transition!(
    Potassium41_767D2, // [Tiecke 2019]
    391_016_406_210_000.0, // 4 2S1/2 - 4 2P3/2 transition
    6.035e6, // Natural Line Width
    17.5, // Saturation intensity W/m^2 = 10*mW/cm^2
    BOHRMAG,
    -BOHRMAG,
    0.0
); 

species!(Cesium133, Cesium133_852D2, 133);
transition!(
    Cesium133_852D2, // [Tiecke 2019]
    351_725_718_500_000.0, // 4 2S1/2 - 4 2P3/2 transition
    5.22e6, // Natural Line Width
    11.0, // Saturation intensity W/m^2 = 10*mW/cm^2
    1.33*BOHRMAG,
    -1.33*BOHRMAG,
    0.0
);

species!(Dysprosium164, Dysprosium164_421, 164);
transition!(
    Dysprosium164_421, // [Tiecke 2019]
    711_604_000_000_000.0, // 4 2S1/2 - 4 2P3/2 transition
    32.2e6, // Natural Line Width
    564.0, // Saturation intensity W/m^2 = 10*mW/cm^2
    1.24*BOHRMAG,
    -1.24*BOHRMAG,
    0.0
);

transition!(
    Dysprosium164_626, // [Tiecke 2019]
    478_839_000_000_000.0, // 4 2S1/2 - 4 2P3/2 transition
    136e3, // Natural Line Width
    0.720, // Saturation intensity W/m^2 = 10*mW/cm^2
    1.24*BOHRMAG,
    -1.24*BOHRMAG,
    0.0
);

species!(Silver109, Silver109_328, 109);
transition!(
    Silver109_328, 
    913_550_191_673_624.0, // 5 2S1/2 - 5 2P3/2 transition
    23.4e6, // Natural Line Width
    870.0, // Saturation intensity W/m^2 = 10*mW/cm^2
    1.*BOHRMAG,
    -1.*BOHRMAG,
    0.0
);
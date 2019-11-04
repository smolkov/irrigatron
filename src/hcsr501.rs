/// # RPI HC-SR501
/// Infrarot Bewegungsmelder - ideal für die Verwendung mit dem Raspberry Pi. 
/// 
/// Technische Daten
/// Betriebsspannung: 5 ... 20 V DC
/// Ruhestrom: 65 µA
/// Level der Ausgangsspannung: 3,3V high / 0 V low
/// Verzögerungszeit einstellbar (0,3 ... 18 Sekunden)
/// Schaltzeit (einstellbar): 5 ... 200 Sekunden
/// Reichweite (einstellbar): 3 ... 7 Meter
/// Blockade-Zeit: 2,5 Sekunden
/// Erfassungsbereich: 140°
/// Betriebstemperatur: -15 ... 70°
/// Abmessungen Platine: 32 x 24 mm
/// Schrauben Lochabstand: 28 mm
/// Lochdurchmesser: 2 mm
/// Durchmesser Sensorlinse: 23 mm
/// 
/// Trigger Methoden: L unrepeatable trigger, H repeatable trigger
/// 
/// Trigger kann per Jumper einfach umgeschaltet werden!
/// 
/// Aufbau
/// Der Aufbau ist sehr einfach, da nur ein Pin bei Bewegung aktiviert werden muss. Die Pins am PIR sind beschriftet:
/// 
/// VCC an Pin 2 (5V)
/// OUT an Pin 16 (GPIO 23)
/// GND an Pin 6 (Ground)

extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

/// HcSR501 device
pub struct HcSr501{
    /// Echo
    echo: Pin,

}
//  let trigger = Pin::new(62);
// let echo    = Pin::new(63);
impl HcSr501 {
    fn new(echo: Pin) -> HcSr501 {
        let mut echo = echo;
        echo.set_direction(Direction::In).unwrap();
        HcSr501 {
           echo: echo,
        } 
    }
}



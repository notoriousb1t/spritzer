use std::fmt::Display;

#[allow(dead_code)]
pub(crate) const X0_NO_DAMAGE: u8 = 0;
#[allow(dead_code)]
pub(crate) const X1: u8 = 1;
#[allow(dead_code)]
pub(crate) const X2: u8 = 2;
#[allow(dead_code)]
pub(crate) const X3: u8 = 3;
#[allow(dead_code)]
pub(crate) const X4: u8 = 4;
#[allow(dead_code)]
pub(crate) const X5: u8 = 5;
#[allow(dead_code)]
pub(crate) const X6: u8 = 6;
#[allow(dead_code)]
pub(crate) const X7: u8 = 7;
#[allow(dead_code)]
pub(crate) const X8: u8 = 8;
#[allow(dead_code)]
pub(crate) const X9_GANON: u8 = 9;

#[derive(Clone, Copy)]
pub(crate) struct DamageClass {
    pub(crate) green_mail: u8,
    pub(crate) blue_mail: u8,
    pub(crate) red_mail: u8,
}

impl DamageClass {
    pub(crate) fn default() -> Self {
        DamageClass {
            green_mail: 0,
            blue_mail: 0,
            red_mail: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct DamageSubclass {
    pub(crate) boomerang: u8,
    pub(crate) sword1: u8,
    pub(crate) sword2: u8,
    pub(crate) sword3: u8,
    pub(crate) sword4: u8,
    pub(crate) sword5: u8,
    pub(crate) arrow1: u8,
    pub(crate) hookshot: u8,
    pub(crate) bomb: u8,
    pub(crate) arrow2: u8,
    pub(crate) powder: u8,
    pub(crate) fire_rod: u8,
    pub(crate) ice_rod: u8,
    pub(crate) bombos: u8,
    pub(crate) ether: u8,
    pub(crate) quake: u8,
}

impl DamageSubclass {
    pub(crate) fn default() -> Self {
        DamageSubclass {
            boomerang: 0,
            sword1: 0,
            sword2: 0,
            sword3: 0,
            sword4: 0,
            sword5: 0,
            arrow1: 0,
            hookshot: 0,
            bomb: 0,
            arrow2: 0,
            powder: 0,
            fire_rod: 0,
            ice_rod: 0,
            bombos: 0,
            ether: 0,
            quake: 0,
        }
    }
}

impl Display for DamageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Damage Class:
green_mail = {}
blue_mail = {}
red_mail = {}
"#,
            self.green_mail, self.blue_mail, self.red_mail,
        )
    }
}

impl Display for DamageSubclass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Damage Subclass:
  boomerang = {}
  sword1 = {} 
  sword2 = {} 
  sword3 = {} 
  sword4 = {} 
  sword5 = {} 
  arrow1 = {} 
  hookshot = {} 
  bomb = {} 
  arrow2 = {} 
  powder = {} 
  fire_rod = {} 
  ice_rod = {} 
  bombos = {} 
  ether = {} 
  quake = {}
"#,
            self.boomerang,
            self.sword1,
            self.sword2,
            self.sword3,
            self.sword4,
            self.sword5,
            self.arrow1,
            self.hookshot,
            self.bomb,
            self.arrow2,
            self.powder,
            self.fire_rod,
            self.ice_rod,
            self.bombos,
            self.ether,
            self.quake,
        )
    }
}

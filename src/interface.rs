use std::slice;

use ::endpoint::Endpoint;


/// Describes an interface.
#[derive(Debug,PartialEq)]
pub struct Interface {
  number: u8,
  settings: Vec<InterfaceSetting>
}

impl Interface {
  /// Returns the interfaces number.
  pub fn number(&self) -> u8 {
    self.number
  }

  /// Returns a collection of the interface's alternate settings.
  pub fn settings(&self) -> &[InterfaceSetting] {
    &self.settings[..]
  }
}


/// Describes an alternate setting for an interface.
#[derive(Debug,PartialEq)]
pub struct InterfaceSetting {
  number: u8,
  interface_class: u8,
  interface_sub_class: u8,
  interface_protocol: u8,
  //description_index: u8,
  endpoints: Vec<Endpoint>
}

impl InterfaceSetting {
  /// Returns the setting number.
  pub fn number(&self) -> u8 {
    self.number
  }

  /// Returns the interface's class code.
  pub fn class_code(&self) -> u8 {
    self.interface_class
  }

  /// Returns the interface's sub class code.
  pub fn sub_class_code(&self) -> u8 {
    self.interface_sub_class
  }

  /// Returns the interface's protocol code.
  pub fn protocol_code(&self) -> u8 {
    self.interface_protocol
  }

  /// Returns a collection of the interface's endpoints.
  pub fn endpoints(&self) -> &[Endpoint] {
    &self.endpoints[..]
  }
}


// Not exported outside the crate.
pub fn from_libusb(interface: &::ffi::libusb_interface) -> Interface {
  let settings = unsafe { slice::from_raw_parts(interface.altsetting, interface.num_altsetting as usize) };
  debug_assert!(settings.len() > 0);

  let iface_number = settings[0].bInterfaceNumber;

  Interface {
    number: iface_number,
    settings: settings.iter().map(|setting| {
      debug_assert_eq!(iface_number, setting.bInterfaceNumber);

      let endpoints = unsafe { slice::from_raw_parts(setting.endpoint, setting.bNumEndpoints as usize) };

      InterfaceSetting {
        number:              setting.bAlternateSetting,
        interface_class:     setting.bInterfaceClass,
        interface_sub_class: setting.bInterfaceSubClass,
        interface_protocol:  setting.bInterfaceProtocol,
        //description_index:   0,
        endpoints:           endpoints.iter().map(|endpoint| ::endpoint::from_libusb(&endpoint)).collect()
      }
    }).collect()
  }
}


#[cfg(test)]
mod test {
  #[test]
  fn it_has_interface_number() {
    assert_eq!(42, ::interface::from_libusb(&interface!(interface_descriptor!(bInterfaceNumber: 42))).number());
  }

  #[test]
  fn it_has_alternate_setting_number() {
    assert_eq!(vec!(42), ::interface::from_libusb(&interface!(interface_descriptor!(bAlternateSetting: 42))).settings().iter().map(|setting| setting.number()).collect::<Vec<_>>());
  }

  #[test]
  fn it_has_class_code() {
    assert_eq!(vec!(42), ::interface::from_libusb(&interface!(interface_descriptor!(bInterfaceClass: 42))).settings().iter().map(|setting| setting.class_code()).collect::<Vec<_>>());
  }

  #[test]
  fn it_has_sub_class_code() {
    assert_eq!(vec!(42), ::interface::from_libusb(&interface!(interface_descriptor!(bInterfaceSubClass: 42))).settings().iter().map(|setting| setting.sub_class_code()).collect::<Vec<_>>());
  }

  #[test]
  fn it_has_protocol_code() {
    assert_eq!(vec!(42), ::interface::from_libusb(&interface!(interface_descriptor!(bInterfaceProtocol: 42))).settings().iter().map(|setting| setting.protocol_code()).collect::<Vec<_>>());
  }

  #[test]
  fn it_has_endpoints() {
    let endpoint = endpoint_descriptor!(
      bEndpointAddress: 0x87,
      bmAttributes:     0x03,
      wMaxPacketSize:   42
    );

    assert_eq!(
      vec!(&::endpoint::from_libusb(&endpoint)),
      ::interface::from_libusb(&interface!(interface_descriptor!(endpoint))).settings()[0].endpoints().iter().collect::<Vec<_>>()
    );
  }
}

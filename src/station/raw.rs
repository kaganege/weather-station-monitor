use core::mem::{align_of, size_of};

macro_rules! declare_field {
    ($tag:literal $data:ty [$size:literal] => $field_name:ident $(($doc:literal))?) => {
        $(#[doc = $doc])?
        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub(super) struct $field_name {
            pub data: [u8; Self::SIZE],
        }

        impl $field_name {
            #[inline]
            pub const fn invalid() -> Self {
                Self {
                    data: [b'.'; Self::SIZE],
                }
            }

            #[inline]
            pub const fn new(data: [u8; Self::SIZE]) -> Self {
                Self { data }
            }
        }

        impl Default for $field_name {
            fn default() -> Self {
                Self::invalid()
            }
        }

        impl TryFrom<RawField<{ $field_name::SIZE }>> for $field_name {
            type Error = WrongFieldTagError;

            fn try_from(field: RawField<{ Self::SIZE }>) -> Result<Self, Self::Error> {
                (field.tag == Self::TAG)
                    .then_some(Self::new(field.data))
                    .ok_or(WrongFieldTagError)
            }
        }

        impl From<$field_name> for RawField<{ $field_name::SIZE }> {
            fn from(field: $field_name) -> Self {
                Self {
                    tag: $field_name::TAG,
                    data: field.data,
                }
            }
        }

        impl Field for $field_name {
            type ActualType = $data;

            const TAG: u8 = $tag;
            const SIZE: usize = $size;
        }
    };
}

pub(super) trait Field {
    const TAG: u8;
    const SIZE: usize;

    type ActualType;
}

#[derive(Debug, thiserror::Error)]
#[error("wrong field tag")]
pub struct WrongFieldTagError;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RawField<const N: usize> {
    pub tag: u8,
    pub data: [u8; N],
}

impl<const N: usize> RawField<N> {
    #[inline]
    pub fn is_valid(&self) -> bool {
        !self.data.contains(&b'.')
    }
}

declare_field!(b'c' u16[3] => RawWindDirection ("air direction, degree"));
declare_field!(b's' u16[3] => RawWindSpeed1Min ("air speed(1 minute), 0.1 miles per hour"));
declare_field!(b'g' u16[3] => RawWindSpeed5Min ("air speed(5 minutes), 0.1 miles per hour"));
declare_field!(b't' u16[3] => RawTemperature   ("temperature, Fahrenheit"));
declare_field!(b'r' u16[3] => RawRainfall1Hour ("rainfall(1 hour), 0.01 inches"));
declare_field!(b'p' u16[3] => RawRainfall1Day  ("rainfall(24 hours), 0.01 inches"));
declare_field!(b'h'  u8[2] => RawHumidity      ("humidity, %"));
declare_field!(b'b' u32[5] => RawAirPressure   ("atmosphere, 0.1 hpa"));

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RawBreak {
    tag: [u8; 4],
}

impl RawBreak {
    pub const SIZE: usize = 4;
    pub const TAG: [u8; Self::SIZE] = *br"\r\n";
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RawData {
    /// c000: air direction, degree
    pub wind_direction: RawField<{ RawWindDirection::SIZE }>,
    /// s000: air speed(1 minute), 0.1 miles per hour
    pub wind_speed_1_min: RawField<{ RawWindSpeed1Min::SIZE }>,
    /// g000: air speed(5 minutes), 0.1 miles per hour
    pub wind_speed_5_min: RawField<{ RawWindSpeed5Min::SIZE }>,
    /// t086: temperature, Fahrenheit
    pub temperature: RawField<{ RawTemperature::SIZE }>,
    /// r000: rainfall(1 hour), 0.01 inches
    pub rainfall_1_hour: RawField<{ RawRainfall1Hour::SIZE }>,
    /// p000: rainfall(24 hours), 0.01 inches
    pub rainfall_1_day: RawField<{ RawRainfall1Day::SIZE }>,
    /// h53: humidity, % (00％ = 100)
    pub humidity: RawField<{ RawHumidity::SIZE }>,
    /// b10020: atmosphere, 0.1 hpa
    pub air_pressure: RawField<{ RawAirPressure::SIZE }>,
    /// CR/LF
    pub r#break: [u8; RawBreak::SIZE],
}

const _: () = {
    assert!(align_of::<RawData>() == 1);
    assert!(size_of::<RawData>() == 37);
};

impl RawData {
    pub const fn from_slice(bytes: &[u8; size_of::<Self>()]) -> &Self {
        unsafe { &*(bytes.as_ptr() as *const Self) }
    }
}

use core::ops::{Add, AddAssign, Sub, SubAssign};

use bit_field::BitField;

use crate::math;

/// # Virtual Address
///
///
/// On `x86_64`, only the 48 lower bits of a virtual address can be used. The top 16 bits need
/// to be copies of bit 47, i.e. the most significant bit. Addresses that fulfil this criterium
/// are called “canonical”. This type guarantees that it always represents a canonical address.
/// ## Source
/// The idea to implement said structure comes from the `x86_64` crate: https://docs.rs/x86_64/0.14.8/src/x86_64/addr.rs.html#35
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    #[inline(always)]
    pub fn new(addr: u64) -> Self {
        Self::try_new(addr).expect(
            "Creating a new address failed due to the \n\
                                    address not staying in bits 48 to 64",
        )
    }

    #[inline]
    /// # Try New
    /// Tries to create the struct, if any value in the 47..64 area is set, it returns an error
    pub fn try_new(addr: u64) -> Result<VirtualAddress, u64> {
        match addr.get_bits(47..64) {
            0 | 0x1fff => Ok(VirtualAddress(addr)),
            1 => Ok(VirtualAddress::truncate(addr)),
            bad => Err(bad),
        }
    }

    pub const fn const_new_unchecked(addr: u64) -> VirtualAddress {
        VirtualAddress(addr)
    }

    /// # Truncate
    /// Creates a new virtual address, but removes bits 47..64
    pub const fn truncate(addr: u64) -> Self {
        // It will sign extend the value, repeating the leftmost bit.
        Self(((addr << 16) as i64 >> 16) as u64)
    }

    /// # Zero
    /// Creates a new VirtAddr with value 0
    #[inline]
    pub const fn zero() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self::new(ptr as u64)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    pub fn as_ptr<T>(&self) -> *const T {
        self.as_u64() as *const T
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    pub fn as_mut_ptr<T>(&self) -> *mut T {
        self.as_u64() as *mut T
    }

    #[inline]
    pub fn as_ref<'retval, T: Copy>(&self) -> &'retval T {
        unsafe { &*(self.as_ptr()) }
    }

    #[inline]
    pub fn as_mut<'retval, T: Copy>(&self) -> &'retval mut T {
        unsafe { &mut *self.as_mut_ptr() }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn set(&mut self, addr: u64) {
        self.try_set(addr)
            .expect("Failed to set address due to bits 47..64 being occupied")
    }

    #[inline(always)]
    /// # Try Set
    /// Tries to set the value, if any value within the range is set, a tuple is returned
    /// The first value within it contains the original address, the second one contains
    /// the bits, which were bad
    pub fn try_set(&mut self, addr: u64) -> Result<(), (u64, u64)> {
        match addr.get_bits(47..64) {
            0 | 0x1fff => {
                self.0 = addr;
                return Ok(());
            }
            1 => {
                self.0 = ((addr << 16) as i64 >> 16) as u64;
                return Ok(());
            }
            bad => Err((bad, Self::truncate(addr).as_u64())),
        }
    }

    #[inline(always)]
    pub fn align_up_and_get<U>(&self, align: U) -> Self
    where
        U: Into<u64>,
    {
        Self(math::align_up(self.0, align.into()))
    }

    #[inline(always)]
    pub fn align_up_and_set<U>(&mut self, align: U)
    where
        U: Into<u64>,
    {
        self.set(math::align_up(self.0, align.into()))
    }

    #[inline]
    pub fn align_down_and_get<U>(&self, align: U) -> Self
    where
        U: Into<u64>,
    {
        Self(math::align_down(self.0, align.into()))
    }

    #[inline(always)]
    pub fn align_down_and_set<U>(&mut self, align: U)
    where
        U: Into<u64>,
    {
        self.set(math::align_down(self.0, align.into()))
    }

    #[inline(always)]
    pub fn is_aligned<U>(&mut self, align: U) -> bool
    where
        U: Into<u64>,
    {
        self.align_down_and_get(align) == *self
    }
}

impl core::fmt::Debug for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("VirtAddr")
            .field(&format_args!("{:#x}", self.0))
            .finish()
    }
}

impl core::fmt::Binary for VirtualAddress {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.0, f)
    }
}

impl core::fmt::LowerHex for VirtualAddress {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl core::fmt::Octal for VirtualAddress {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Octal::fmt(&self.0, f)
    }
}

impl core::fmt::UpperHex for VirtualAddress {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.0, f)
    }
}

impl core::fmt::Pointer for VirtualAddress {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Pointer::fmt(&(self.0 as *const ()), f)
    }
}

impl Add<u64> for VirtualAddress {
    type Output = Self;
    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        VirtualAddress::new(self.0 + rhs)
    }
}

impl AddAssign<u64> for VirtualAddress {
    #[inline]
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + rhs;
    }
}

#[cfg(target_pointer_width = "64")]
impl Add<usize> for VirtualAddress {
    type Output = Self;
    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        self + rhs as u64
    }
}

#[cfg(target_pointer_width = "64")]
impl AddAssign<usize> for VirtualAddress {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.add_assign(rhs as u64)
    }
}

impl Sub<u64> for VirtualAddress {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: u64) -> Self::Output {
        VirtualAddress::new(self.0.checked_sub(rhs).unwrap())
    }
}

impl SubAssign<u64> for VirtualAddress {
    #[inline]
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self - rhs;
    }
}

#[cfg(target_pointer_width = "64")]
impl Sub<usize> for VirtualAddress {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        self - rhs as u64
    }
}

#[cfg(target_pointer_width = "64")]
impl SubAssign<usize> for VirtualAddress {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.sub_assign(rhs as u64)
    }
}

impl Sub<VirtualAddress> for VirtualAddress {
    type Output = u64;
    #[inline]
    fn sub(self, rhs: VirtualAddress) -> Self::Output {
        self.as_u64().checked_sub(rhs.as_u64()).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
/// # Physical Address
/// Represents an 8-bit address, even on non 64-bit systems
/// `x86_64`, only the 52 lower bits of a physical address can be used. The top 12 bits need
/// to be zero. This type guarantees that it always represents a valid physical address.
/// The idea to implement said structure comes from the `x86_64` crate: https://docs.rs/x86_64/0.14.8/src/x86_64/addr.rs.html#35
pub struct PhysicalAddress(u64);

impl PhysicalAddress {
    #[inline]
    pub fn new(addr: u64) -> Self {
        let ret = if addr.get_bits(52..64) != 0 {
            Self::truncate(addr)
        } else {
            Self(addr)
        };

        assert!(
            ret.0.get_bits(52..64) == 0,
            "Even after truncating, the bits 52..64 were set"
        );
        ret
    }

    #[inline]
    /// # Try New
    /// Tries to create a new value, returns a tuple on error.
    /// The first value within it contains the original address, the second one contains
    /// the bits, which were bad
    pub fn try_new(addr: u64) -> Result<Self, (u64, u64)> {
        if addr.get_bits(52..64) != 0 {
            Err((addr.get_bits(52..64), Self::truncate(addr).as_u64()))
        } else {
            Ok(Self(addr))
        }
    }

    #[inline]
    pub const fn truncate(addr: u64) -> Self {
        Self(addr % (1 << 52))
    }

    #[inline]
    pub const fn zero() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    #[inline]
    pub const fn is_null(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn set(&mut self, addr: u64) {
        self.try_set(addr)
            .expect("Failed to set the address due to bits 47..56 being occupied");
    }

    /// # Try Set
    /// Tries to set the address, returns a tuple on error.
    /// The first value within it contains the original address, the second one contains
    /// the bits, which were bad
    #[inline]
    pub fn try_set(&mut self, addr: u64) -> Result<(), (u64, u64)> {
        if addr.get_bits(52..64) != 0 {
            let new = Self::truncate(addr).as_u64();
            if new.get_bits(52..64) != 0 {
                Err((addr, Self::truncate(new).as_u64()))
            } else {
                self.0 = new;
                Ok(())
            }
        } else {
            self.0 = addr;
            Ok(())
        }
    }

    #[inline]
    pub fn align_up_and_get<U>(&self, align: U) -> Self
    where
        U: Into<u64>,
    {
        Self(math::align_up(self.0, align.into()))
    }

    #[inline]
    pub fn align_down_and_get<U>(self, align: U) -> Self
    where
        U: Into<u64>,
    {
        Self(math::align_down(self.0, align.into()))
    }

    #[inline]
    pub fn align_up_and_set<U>(&mut self, align: U)
    where
        U: Into<u64>,
    {
        self.set(math::align_up(self.0, align.into()))
    }

    #[inline]
    pub fn align_down_and_set<U>(&mut self, align: U)
    where
        U: Into<u64>,
    {
        self.set(math::align_down(self.0, align.into()))
    }

    #[inline]
    pub fn is_aligned<U>(self, align: U) -> bool
    where
        U: Into<u64>,
    {
        self.align_down_and_get(align) == self
    }
}

impl core::fmt::Debug for PhysicalAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("PhysAddr")
            .field(&format_args!("{:#x}", self.0))
            .finish()
    }
}

impl core::fmt::Binary for PhysicalAddress {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.0, f)
    }
}

impl core::fmt::LowerHex for PhysicalAddress {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl core::fmt::Octal for PhysicalAddress {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Octal::fmt(&self.0, f)
    }
}

impl core::fmt::UpperHex for PhysicalAddress {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.0, f)
    }
}

impl core::fmt::Pointer for PhysicalAddress {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Pointer::fmt(&(self.0 as *const ()), f)
    }
}

impl Add<u64> for PhysicalAddress {
    type Output = Self;
    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        PhysicalAddress::new(self.0 + rhs)
    }
}

impl AddAssign<u64> for PhysicalAddress {
    #[inline]
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + rhs;
    }
}

#[cfg(target_pointer_width = "64")]
impl Add<usize> for PhysicalAddress {
    type Output = Self;
    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        self + rhs as u64
    }
}

#[cfg(target_pointer_width = "64")]
impl AddAssign<usize> for PhysicalAddress {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.add_assign(rhs as u64)
    }
}

impl Sub<u64> for PhysicalAddress {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: u64) -> Self::Output {
        PhysicalAddress::new(self.0.checked_sub(rhs).unwrap())
    }
}

impl SubAssign<u64> for PhysicalAddress {
    #[inline]
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self - rhs;
    }
}

#[cfg(target_pointer_width = "64")]
impl Sub<usize> for PhysicalAddress {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        self - rhs as u64
    }
}

#[cfg(target_pointer_width = "64")]
impl SubAssign<usize> for PhysicalAddress {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.sub_assign(rhs as u64)
    }
}

impl Sub<PhysicalAddress> for PhysicalAddress {
    type Output = u64;
    #[inline]
    fn sub(self, rhs: PhysicalAddress) -> Self::Output {
        self.as_u64().checked_sub(rhs.as_u64()).unwrap()
    }
}

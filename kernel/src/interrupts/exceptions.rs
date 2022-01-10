pub use self::IDTException::*;

use super::interrupt_frame::InterruptFrame;

#[allow(unused)]
pub enum ExceptionType {
    Fault,
    Abort,
    Interrupt,
    Trap,
}

enumtastic::const_enum! {
    pub enum IDTException: usize => {
        DivideByZero = 0x0,
        Debug = 0x1,
        NonMaskable = 0x2,
        Breakpoint = 0x3,
        Overflow = 0x4,
        BoundRangeExceeded = 0x5,
        InvalidOpcode = 0x6,
        DeviceNotAvailable = 0x7,
        DoubleFault = 0x8,
        InvalidTSS = 0xA,
        SegmentNotPresent = 0xB,
        StackSegmentFault = 0xC,
        GeneralProtectionFault = 0xD,
        PageFault = 0xE,
        // 0xF = RESERVED
        X87FloatingPointException = 0x10,
        AlignmentCheck = 0x11,
        MachineCheck = 0x12,
        SIMDFloatingPointException = 0x13,
        VirtualizationException = 0x14,
        ControlProtection = 0x15,
        // 0x16 - 0x1B = RESERVED
        HypervisorInjection = 0x1C,
        VMMCommunicationException = 0x1D,
        SecurityException = 0x1E,
        // 0x1F = RESERVED
        // TripleFault does not have a code
    }

    impl {
        pub fn error_code(me: &Me) -> &str {
             match *me {
                 DivideByZero => "#DE",
                 Debug => "#DB",
                 NonMaskable => "-",
                 Breakpoint => "#BP",
                 Overflow => "#OF",
                 BoundRangeExceeded => "#BR",
                 InvalidOpcode => "#UD",
                 DeviceNotAvailable => "#NM",
                 DoubleFault => "#DF",
                 InvalidTSS => "#TS",
                 SegmentNotPresent => "#NP",
                 StackSegmentFault => "#SS",
                 GeneralProtectionFault => "#GP",
                 PageFault => "#PF",
                 X87FloatingPointException => "MF",
                 AlignmentCheck => "#AC",
                 MachineCheck => "#MC",
                 SIMDFloatingPointException => "#XM",
                 VirtualizationException => "#VE",
                 ControlProtection => "#CP",
                 HypervisorInjection => "#HV",
                 VMMCommunicationException => "#VC",
                 SecurityException => "#SX",
                 _ => "Unknown",
             }
        }

        pub fn type_(me: &Me) -> super::ExceptionType {
             match *me {
                 DivideByZero => todo!(),
                 Debug => todo!(),
                 NonMaskable => todo!(),
                 Breakpoint => todo!(),
                 Overflow => todo!(),
                 BoundRangeExceeded => todo!(),
                 InvalidOpcode => todo!(),
                 DeviceNotAvailable => todo!(),
                 DoubleFault => todo!(),
                 InvalidTSS => todo!(),
                 SegmentNotPresent => todo!(),
                 StackSegmentFault => todo!(),
                 GeneralProtectionFault => todo!(),
                 PageFault => todo!(),
                 X87FloatingPointException => todo!(),
                 AlignmentCheck => todo!(),
                 MachineCheck => todo!(),
                 SIMDFloatingPointException => todo!(),
                 VirtualizationException => todo!(),
                 ControlProtection => todo!(),
                 HypervisorInjection => todo!(),
                 VMMCommunicationException => todo!(),
                 SecurityException => todo!(),
                 _ => todo!(),
             }
        }
    }
}

pub trait Exception<const T: usize> {
    extern "x86-interrupt" fn handle(frame: InterruptFrame);
}
pub struct ExceptionHandler<const T: usize>;

impl Exception<InvalidTSS> for ExceptionHandler<InvalidTSS> {
    extern "x86-interrupt" fn handle(frame: InterruptFrame) {
        panic!("Triggered Fault {} with opcode {}", InvalidTSS, IDTException::error_code(&InvalidTSS))
    }
}
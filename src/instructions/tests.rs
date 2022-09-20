#[macro_export]
#[cfg(test)]
macro_rules! ax_test {
    [$test_name:ident; $($bytes:expr),*; $asserts:expr; ($flags_to_set:expr; $flags_not_to_set:expr)] => {
        // Call the other macro with empty setup code
        ax_test!($test_name; $($bytes),*; |_: &mut Axecutor| {}; $asserts; ($flags_to_set; $flags_not_to_set));
    };
    [$test_name:ident; $($bytes:expr),*; $setup:expr; $asserts:expr; ($flags_to_set:expr; $flags_not_to_set:expr)] => {
		#[test]
		fn $test_name () {
            smol::block_on(async {
                use rand::Rng;
                use crate::instructions::errors::AxError;

                let bytes = &[$($bytes),*];

                // Always use a random rip
                let random_rip = rand::thread_rng().gen::<u64>() & 0x0000_ffff_ffff_ffff;

                let mut ax = Axecutor::new(bytes, random_rip, random_rip).expect("Failed to create axecutor");

                $setup(&mut ax);

                match ax.execute().await {
                    Err(e) => panic!("Failed to execute: {:?}", AxError::from(e)),
                    _ => {}
                };

                let flags = ax.state.rflags;

                $asserts(ax);

                // Check flags
                use crate::instructions::flags::*;
                for flag in FLAG_LIST {
                    // If the flag should be set, it must be != 0
                    if $flags_to_set & flag != 0 {
                        assert!(flags & flag != 0, "FLAG_{} should be set, but wasn't", FLAG_TO_NAMES.get(&flag).expect("Flag not found"));
                    }

                    if $flags_not_to_set & flag != 0 {
                        assert!(flags & flag == 0, "FLAG_{} should not be set, but was", FLAG_TO_NAMES.get(&flag).expect("Flag not found"));
                    }
                }
            });
		}
    };
    [$test_name:ident; $($bytes:expr),*; $asserts:expr] => {
        // Call the other macro with empty setup code
        ax_test!($test_name; $($bytes),*; |_: &mut Axecutor| {}; $asserts; (0; 0));
    };
    [$test_name:ident; $($bytes:expr),*; $setup:expr; $asserts:expr] => {
        // Call the other macro with empty flags
        ax_test!($test_name; $($bytes),*; $setup; $asserts; (0; 0));
    };
}

#[macro_export]
#[cfg(test)]
macro_rules! assert_reg_value {
    [b; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr8(), "Register must be 8 bit wide");
        let val = $axecutor.reg_read_8(wrap);
        assert_eq!(
            &val, &$value,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, &val
        );
    };
    [w; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr16(), "Register must be 16 bit wide");
        let val = $axecutor.reg_read_16(wrap);
        assert_eq!(
            &val, &$value,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, &val
        );
    };
    [d; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr32(), "Register must be 32 bit wide");
        let val = $axecutor.reg_read_32(wrap);
        assert_eq!(
            &val, &$value,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, &val
        );
    };
    [q; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr64() || $reg.is_ip(), "Register must be 64 bit wide");
        let val = $axecutor.reg_read_64(wrap);
        assert_eq!(
            &val, &$value,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, &val
        );
    };
}

#[macro_export]
#[cfg(test)]
macro_rules! write_reg_value {
    (b; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr8(), "Register must be 8 bit wide");
        $axecutor.reg_write_8(wrap, $value);
    };
    (w; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr16(), "Register must be 16 bit wide");
        $axecutor.reg_write_16(wrap, $value);
    };
    (d; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr32(), "Register must be 32 bit wide");
        $axecutor.reg_write_32(wrap, $value);
    };
    (q; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = SupportedRegister::from($reg);
        assert!($reg.is_gpr64(), "Register must be 64 bit wide");
        $axecutor.reg_write_64(wrap, $value);
    };
}

macro_rules! tests {
    (
    $(  use $($use:tt)::*;                                                      )*

        #[test]
    $(  #[strict_handle_check_exception = $strict_handle_check_exception:expr]  )?
        fn $test_name:ident () {
            $($test_body:tt)*
        }

        $($rest:tt)*
    ) => {
        #[cfg(all(std, test))] #[test] fn $test_name() {
            match crate::test::flavor().as_str() {
                "STRICT_HANDLE_CHECK" => {
                    crate::set_process_mitigation_policy(crate::process::mitigation::StrictHandleCheckPolicy {
                        handle_exceptions_permanently_enabled:          true,
                        raise_exception_on_invalid_handle_reference:    true,
                        .. Default::default()
                    }).unwrap();
                    // continue to test body
                },
                "" => {
                    // continue to test body
                },
                _unexpected => panic!("unexpected test flavor: {_unexpected:?}"),
            }
            {
                $(#[allow(unused_imports)] use $($use)::*;)*
                $($test_body)*
            }
            match crate::test::flavor().as_str() {
                "STRICT_HANDLE_CHECK" => {},
                "" => {
                    // run variant tests after main test body
                    $({
                        let test = concat!(module_path!(), "::", stringify!($test_name));
                        let test = test.strip_prefix("firehazard::").unwrap_or(test); // module_path!() includes crate name, test names don't
                        let expected = $strict_handle_check_exception as u32;
                        if let Some(actual) = crate::test::run_one_exact_flavor(test, "STRICT_HANDLE_CHECK") {
                            let actual = actual as u32;
                            assert!(
                                expected == actual,
                                "expected exit code 0x{expected:08X} when running test with strict handle checks, instead got exit code 0x{actual:08X}",
                            );
                        } else {
                            panic!("expected exit code 0x{expected:08X} when running test with strict handle checks, instead got... a *unix* signal? wat?");
                        }
                    })?;
                },
                _unexpected => panic!("unexpected test flavor: {_unexpected:?}"),
            }
        }

        tests!{
            $(use $($use)::*;)*

            $($rest)*
        }
    };

    (
        $(  use $($use:tt)::*;                                                      )*
    ) => {}
}

macro_rules! tests {
    (
    $(  use $($use:tt)::*;                                                      )*

        #[test]
    $(  #[isolate $(@@@@NEVER@@@@ $isolate:tt)? ]                               )?
    $(  #[strict_handle_check_exception = $strict_handle_check_exception:expr]  )?
        fn $test_name:ident () {
            $($test_body:tt)*
        }

        $($rest:tt)*
    ) => {
        #[cfg(all(std, test))] #[test] fn $test_name() {
            let isolate = false $($($isolate)? || true)?;

            let firehazard_mod_test_name = concat!(module_path!(), "::", stringify!($test_name));
            let mod_test_name = firehazard_mod_test_name.strip_prefix("firehazard::").unwrap_or(firehazard_mod_test_name); // module_path!() includes crate name, test names don't
            let test_flavor = crate::test::flavor();
            let test_flavor = &*test_flavor;

            match test_flavor {
                "STRICT_HANDLE_CHECK" => {
                    crate::set_process_mitigation_policy(crate::process::mitigation::StrictHandleCheckPolicy {
                        handle_exceptions_permanently_enabled:          true,
                        raise_exception_on_invalid_handle_reference:    true,
                        .. Default::default()
                    }).unwrap();
                    // continue to test body
                },
                "" if isolate => {
                    let expected = 0x1501A7E; // "ISOLATE" 1337ified (we explicitly exit with this code to ensure the test was actually pattern matched)
                    let actual = crate::test::run_one_exact_flavor(mod_test_name, "ISOLATE").expect("expected exit code 0x{expected:0X} when running test under isolation, instead got... a *unix* signal? wat?");
                    assert!(expected == actual, "expected exit code 0x{expected:0X} when running test with strict handle checks, instead got exit code 0x{actual:08X}");
                    return;
                },
                "ISOLATE" | "" => {
                    // continue to test body
                },
                _unexpected => panic!("unexpected test flavor: {_unexpected:?}"),
            }
            {
                $(#[allow(unused_imports)] use $($use)::*;)*
                $($test_body)*
            }
            match test_flavor {
                "STRICT_HANDLE_CHECK"   => std::process::exit(0x5721C7),    // 1337ified "STRICT"
                "ISOLATE"               => std::process::exit(0x1501A7E),   // 1337ified "ISOLATE"
                "" => {
                    // run variant tests after main test body
                    $({
                        let expected = $strict_handle_check_exception as u32;
                        let real_expected = if expected == 0 { 0x5721C7 } else { expected };
                        if let Some(actual) = crate::test::run_one_exact_flavor(mod_test_name, "STRICT_HANDLE_CHECK") {
                            let actual = actual as u32;
                            assert!(
                                real_expected == actual,
                                "expected exit code 0x{expected:08X} when running test with strict handle checks, instead got exit code 0x{actual:08X}",
                            );
                        } else {
                            panic!("expected exit code 0x{expected:08X} when running test with strict handle checks, instead got... a *unix* signal? wat?");
                        }
                    })?
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

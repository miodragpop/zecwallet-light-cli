pub fn get_closest_checkpoint(chain_name: &str, height: u64) ->  Option<(u64, &'static str, &'static str)> {
    log::info!("Trying to get checkpoint closest to block {}", height);
    match chain_name {
        "test" => get_test_checkpoint(height),
        "main" => get_main_checkpoint(height),
        _      => None
    }
}

fn get_test_checkpoint(height: u64) ->  Option<(u64, &'static str, &'static str)> {
    let checkpoints: Vec<(u64, &str, &str)> = vec![
        (600000, "0107385846c7451480912c294b6ce1ee1feba6c2619079fd9104f6e71e4d8fe7",
                 "01690698411e3f8badea7da885e556d7aba365a797e9b20b44ac0946dced14b23c001001ab2a18a5a86aa5d77e43b69071b21770b6fe6b3c26304dcaf7f96c0bb3fed74d000186482712fa0f2e5aa2f2700c4ed49ef360820f323d34e2b447b78df5ec4dfa0401a332e89a21afb073cb1db7d6f07396b56a95e97454b9bca5a63d0ebc575d3a33000000000001c9d3564eff54ebc328eab2e4f1150c3637f4f47516f879a0cfebdf49fe7b1d5201c104705fac60a85596010e41260d07f3a64f38f37a112eaef41cd9d736edc5270145e3d4899fcd7f0f1236ae31eafb3f4b65ad6b11a17eae1729cec09bd3afa01a000000011f8322ef806eb2430dc4a7a41c1b344bea5be946efc7b4349c1c9edb14ff9d39"
        ),
        (650000, "003f7e09a357a75c3742af1b7e1189a9038a360cebb9d55e158af94a1c5aa682",
                 "010113f257f93a40e25cfc8161022f21c06fa2bc7fb03ee9f9399b3b30c636715301ef5b99706e40a19596d758bf7f4fd1b83c3054557bf7fab4801985642c317d41100001b2ad599fd7062af72bea99438dc5d8c3aa66ab52ed7dee3e066c4e762bd4e42b0001599dd114ec6c4c5774929a342d530bf109b131b48db2d20855afa9d37c92d6390000019159393c84b1bf439d142ed2c54ee8d5f7599a8b8f95e4035a75c30b0ec0fa4c0128e3a018bd08b2a98ed8b6995826f5857a9dc2777ce6af86db1ae68b01c3c53d0000000001e3ec5d790cc9acc2586fc6e9ce5aae5f5aba32d33e386165c248c4a03ec8ed670000011f8322ef806eb2430dc4a7a41c1b344bea5be946efc7b4349c1c9edb14ff9d39"
        )
    ];

    find_checkpoint(height, checkpoints)
}


fn get_main_checkpoint(height: u64) ->  Option<(u64, &'static str, &'static str)> {
    let checkpoints: Vec<(u64, &str, &str)> = vec![
        (1080000, "000239bee6cacde8de13e9a15684c2f253fb86ba6ed27ca6bd32a4ac11219303",
                 "01cf2fe760da20ca81bb08e2d6d58086e3499e94b359187909ef82e8e7ce717d4b00110001e2c8f8791f21704fa3e263e33199dc13e9994bcc0d4feddb0b49ca4e034b743f000001567af0c591f3e447bbf7cb14cdd9a029419c75e77f9489bce9b884965174b95100000001b4f8e50bb7a59b9ddf28ea44d3f6b96ceffdf7ac47ded46be5e794763aa0816f0001c6d372a5b1deb3533bcef474a082ac602ab5aea9ab3e816e44c5c7b45c2d444f0126a4132b19084ec2963243218856ad3ffb8f6b540540ab8ef041e288a09b1c5801d3e2cb6b8be27bb1ca9a29c7d43e9f803ea9e8d32a4e264f99f778807b40761a000000018394f4a5964d8b8048cfd1e36eac3b679c50fdf2f3d613c09e5b542b11b16444"
        ),
        (1090000, "000513d5c14076d2e8beed5ffe7f605a395c4b2848990f1b12700252130169cf",
                "011d097b90c1cb9c87ef55f58660054dd3b539876ecff5f91726afd16a99c3694e00110000000001dbc5baf91cfb72ebfe8f0f1a2a65335c1a9389cc4dc10020c8333a350b4286200116237eebabdcdf57f4c6f2b9f72875ee7d174cb895ed6b90ef74796ddee8601e000168221008f100ca83a74a9fb7775740818eba9fd494a4498e345472358432c365000000000001641e6287a68a6dac80dc54025eb527ccdc60f91258a3c48290719355d2f9034a0000018394f4a5964d8b8048cfd1e36eac3b679c50fdf2f3d613c09e5b542b11b16444"
    ),
    ];

    find_checkpoint(height, checkpoints)
}

fn find_checkpoint(height: u64, chkpts: Vec<(u64, &'static str, &'static str)>) -> Option<(u64, &'static str, &'static str)> {
    // Find the closest checkpoint
    let mut heights = chkpts.iter().map(|(h, _, _)| *h as u64).collect::<Vec<_>>();
    heights.sort();

    match get_first_lower_than(height, heights) {
        Some(closest_height) => {
            chkpts.iter().find(|(h, _, _)| *h ==  closest_height).map(|t| *t)
        },
        None    => None
    }
}

fn get_first_lower_than(height: u64, heights: Vec<u64>) -> Option<u64> {
    // If it's before the first checkpoint, return None. 
    if heights.len() == 0 || height < heights[0] {
        return None;
    }

    for (i, h) in heights.iter().enumerate() {
        if height < *h {
            return Some(heights[i-1]);
        }
    }

    return Some(*heights.last().unwrap());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_lower_than() {
        assert_eq!(get_first_lower_than( 9, vec![10, 30, 40]), None);
        assert_eq!(get_first_lower_than(10, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(11, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(29, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(30, vec![10, 30, 40]).unwrap(), 30);
        assert_eq!(get_first_lower_than(40, vec![10, 30, 40]).unwrap(), 40);
        assert_eq!(get_first_lower_than(41, vec![10, 30, 40]).unwrap(), 40);
        assert_eq!(get_first_lower_than(99, vec![10, 30, 40]).unwrap(), 40);
    }

    #[test]
    fn test_checkpoints() {
        assert_eq!(get_test_checkpoint(500000), None);
        assert_eq!(get_test_checkpoint(600000).unwrap().0, 600000);
        assert_eq!(get_test_checkpoint(625000).unwrap().0, 600000);
        assert_eq!(get_test_checkpoint(650000).unwrap().0, 650000);
        assert_eq!(get_test_checkpoint(655000).unwrap().0, 650000);

        assert_eq!(get_main_checkpoint(500000), None);
        assert_eq!(get_main_checkpoint(610000).unwrap().0, 610000);
        assert_eq!(get_main_checkpoint(625000).unwrap().0, 610000);
    }

}
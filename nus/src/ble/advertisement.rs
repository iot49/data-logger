// https://jimmywongiot.com/2019/08/13/advertising-payload-format-on-ble/

const NAME: &[u8; 6] = b"RV Log";

pub fn adv_data() -> &[u8] {
    #[rustfmt::skip]
    let adv_data: &[u8; 3+18+2+NAME.len()] = &concat_arrays!(
        [ 2u8, raw::BLE_GAP_AD_TYPE_FLAGS as u8, raw::BLE_GAP_ADV_FLAGS_LE_ONLY_GENERAL_DISC_MODE as u8, ],
        [ (NUS_SVC_UUID.len()+1) as u8, raw::BLE_GAP_AD_TYPE_128BIT_SERVICE_UUID_COMPLETE as u8 ], *NUS_SVC_UUID,
        [ (NAME.len() + 1) as u8, raw::BLE_GAP_AD_TYPE_COMPLETE_LOCAL_NAME as u8 ], *NAME
    );

}

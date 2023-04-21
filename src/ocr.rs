#![allow(unused)]

use leptess::LepTess;
use std::collections::HashSet;
use std::io::Cursor;
use std::sync::MutexGuard;

pub fn ocr(path: &str) -> String {
    return ocr_from_disk(path, &mut get_api());
}
pub fn get_api() -> LepTess {
    let mut api = leptess::LepTess::new(Some("models/traineddata/tessdata_fast"), "eng").unwrap();
    api.set_variable(leptess::Variable::TesseditPagesegMode, "3")
        .expect("Failed to set tesseract variable");
    api.set_variable(leptess::Variable::TesseditOcrEngineMode, "2")
        .expect("Failed to set tesseract variable");
    // api.set_variable(leptess::Variable::UserDefinedDpi, "100")
    // .expect("Failed to set tesseract variable");
    return api;
}

fn ocr_from_disk(path: &str, api: &mut LepTess) -> String {
    api.set_image(path).unwrap();
    return api.get_utf8_text().expect("OCR failed");
}

pub fn threaded_ocr(path: &str, mut api: MutexGuard<LepTess>) -> String {
    api.set_image(path).unwrap();
    return api.get_utf8_text().expect("Threaded OCR failed");
}

// enum OcrEngineMode {
//     OEM_TESSERACT_ONLY = 0
//     OEM_LSTM_ONLY = 1
//     OEM_TESSERACT_LSTM_COMBINED = 2
//     OEM_DEFAULT = 3
//     OEM_COUNT = 4
// }
// enum PageSegMode {
//     PSM_OSD_ONLY = 0,
//     PSM_AUTO_OSD = 1,
//     PSM_AUTO_ONLY = 2,
//     PSM_AUTO = 3,
//     PSM_SINGLE_COLUMN = 4,
//     PSM_SINGLE_BLOCK_VERT_TEXT = 5,
//     PSM_SINGLE_BLOCK = 6,
//     PSM_SINGLE_LINE = 7,
//     PSM_SINGLE_WORD = 8,
//     PSM_CIRCLE_WORD = 9,
//     PSM_SINGLE_CHAR = 10,
//     PSM_SPARSE_TEXT,
//     PSM_SPARSE_TEXT_OSD = 12,
//     PSM_RAW_LINE = 13,
//     PSM_COUNT,
// }

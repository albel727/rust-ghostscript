use gs_sys::display as disp;

bitflags! {
    #[derive(Default)]
    pub struct DisplayFormat: disp::DisplayFormat {
        const COLORS_NATIVE = disp::DISPLAY_COLORS_NATIVE;
        const COLORS_GRAY = disp::DISPLAY_COLORS_GRAY;
        const COLORS_RGB = disp::DISPLAY_COLORS_RGB;
        const COLORS_CMYK = disp::DISPLAY_COLORS_CMYK;
        const COLORS_SEPARATION = disp::DISPLAY_COLORS_SEPARATION;
        const MASK_COLORS = disp::DISPLAY_COLORS_MASK;

        const ALPHA_NONE = disp::DISPLAY_ALPHA_NONE;
        const ALPHA_FIRST = disp::DISPLAY_ALPHA_FIRST;
        const ALPHA_LAST = disp::DISPLAY_ALPHA_LAST;
        const UNUSED_FIRST = disp::DISPLAY_UNUSED_FIRST;
        const UNUSED_LAST = disp::DISPLAY_UNUSED_LAST;
        const MASK_ALPHA = disp::DISPLAY_ALPHA_MASK;

        const DEPTH_1 = disp::DISPLAY_DEPTH_1;
        const DEPTH_2 = disp::DISPLAY_DEPTH_2;
        const DEPTH_4 = disp::DISPLAY_DEPTH_4;
        const DEPTH_8 = disp::DISPLAY_DEPTH_8;
        const DEPTH_12 = disp::DISPLAY_DEPTH_12;
        const DEPTH_16 = disp::DISPLAY_DEPTH_16;
        const MASK_DEPTH = disp::DISPLAY_DEPTH_MASK;

        const BIG_ENDIAN = disp::DISPLAY_BIGENDIAN;
        const LITTLE_ENDIAN = disp::DISPLAY_LITTLEENDIAN;
        const MASK_ENDIANNESS = disp::DISPLAY_ENDIAN_MASK;

        const TOP_FIRST = disp::DISPLAY_TOPFIRST;
        const BOTTOM_FIRST = disp::DISPLAY_BOTTOMFIRST;
        const MASK_FIRST_ROW = disp::DISPLAY_FIRSTROW_MASK;

        const NATIVE_555 = disp::DISPLAY_NATIVE_555;
        const NATIVE_565 = disp::DISPLAY_NATIVE_565;
        const MASK_NATIVE_16_BIT = disp::DISPLAY_555_MASK;

        const ROW_ALIGN_DEFAULT = disp::DISPLAY_ROW_ALIGN_DEFAULT;
        const ROW_ALIGN_4 = disp::DISPLAY_ROW_ALIGN_4;
        const ROW_ALIGN_8 = disp::DISPLAY_ROW_ALIGN_8;
        const ROW_ALIGN_16 = disp::DISPLAY_ROW_ALIGN_16;
        const ROW_ALIGN_32 = disp::DISPLAY_ROW_ALIGN_32;
        const ROW_ALIGN_64 = disp::DISPLAY_ROW_ALIGN_64;
        const MASK_ROW_ALIGN = disp::DISPLAY_ROW_ALIGN_MASK;
    }
}

impl DisplayFormat {
    pub fn format_as_init_arg(&self) -> String {
        format!("-dDisplayFormat=16#{:x}", self.bits())
    }
}

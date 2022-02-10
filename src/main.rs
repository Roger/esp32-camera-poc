mod esp_camera;

use std::io::Cursor;
use std::thread;
use std::{sync::Arc, time::Duration};

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};

const CAM_PIN_PWDN: ::std::os::raw::c_int = 32;
const CAM_PIN_RESET: ::std::os::raw::c_int = -1; //software reset will be performed
const CAM_PIN_XCLK: ::std::os::raw::c_int = 0;
const CAM_PIN_SIOD: ::std::os::raw::c_int = 26;
const CAM_PIN_SIOC: ::std::os::raw::c_int = 27;
const CAM_PIN_D7: ::std::os::raw::c_int = 35;
const CAM_PIN_D6: ::std::os::raw::c_int = 34;
const CAM_PIN_D5: ::std::os::raw::c_int = 39;
const CAM_PIN_D4: ::std::os::raw::c_int = 36;
const CAM_PIN_D3: ::std::os::raw::c_int = 21;
const CAM_PIN_D2: ::std::os::raw::c_int = 19;
const CAM_PIN_D1: ::std::os::raw::c_int = 18;
const CAM_PIN_D0: ::std::os::raw::c_int = 5;
const CAM_PIN_VSYNC: ::std::os::raw::c_int = 25;
const CAM_PIN_HREF: ::std::os::raw::c_int = 23;
const CAM_PIN_PCLK: ::std::os::raw::c_int = 22;

fn init_camera() {
    let config = esp_camera::camera_config_t {
        pin_pwdn: CAM_PIN_PWDN,
        pin_reset: CAM_PIN_RESET,
        pin_xclk: CAM_PIN_XCLK,
        pin_sscb_sda: CAM_PIN_SIOD,
        pin_sscb_scl: CAM_PIN_SIOC,

        pin_d7: CAM_PIN_D7,
        pin_d6: CAM_PIN_D6,
        pin_d5: CAM_PIN_D5,
        pin_d4: CAM_PIN_D4,
        pin_d3: CAM_PIN_D3,
        pin_d2: CAM_PIN_D2,
        pin_d1: CAM_PIN_D1,
        pin_d0: CAM_PIN_D0,
        pin_vsync: CAM_PIN_VSYNC,
        pin_href: CAM_PIN_HREF,
        pin_pclk: CAM_PIN_PCLK,

        xclk_freq_hz: 20000000,
        ledc_timer: esp_camera::ledc_timer_t_LEDC_TIMER_0,
        ledc_channel: esp_camera::ledc_channel_t_LEDC_CHANNEL_0,
        pixel_format: esp_camera::pixformat_t_PIXFORMAT_JPEG,
        frame_size: esp_camera::framesize_t_FRAMESIZE_QVGA,
        jpeg_quality: 8,
        fb_count: 1,
        fb_location: esp_camera::camera_fb_location_t_CAMERA_FB_IN_PSRAM,
        grab_mode: esp_camera::camera_grab_mode_t_CAMERA_GRAB_WHEN_EMPTY,
    };

    unsafe {
        esp_camera::esp_camera_init(&config);
    }
}

fn take_picture() {
    unsafe {
        let pic = esp_camera::esp_camera_fb_get();
        let mut buf: *mut u8 = libc::malloc(std::mem::size_of::<u8>()) as *mut u8;
        let mut buf_len = 0;

        esp_camera::frame2bmp(pic, &mut buf, &mut buf_len);

        let slice = std::slice::from_raw_parts_mut(buf, buf_len);

        // println!("Read");
        let img = ImageReader::new(Cursor::new(slice))
            .with_guessed_format()
            .unwrap();

        // println!("Decode");
        match img.decode() {
            Ok(img) => image_to_ascii(img),
            Err(err) => {
                dbg!(err);
            }
        };
        libc::free(buf as *mut libc::c_void);
        esp_camera::esp_camera_fb_return(pic);
    }
}

fn image_to_ascii(image: DynamicImage) {
    let resolution = 8;
    let pallete: [char; 7] = [' ', '.', '/', '*', '#', '$', '@'];

    let mut y = 0;
    let small_img = image.resize_exact(
        image.width() / (resolution / 2),
        image.height() / resolution,
        FilterType::Nearest,
    );

    for p in small_img.pixels() {
        if y != p.1 {
            println!();
            y = p.1;
        }

        let r = p.2 .0[0] as f32;
        let g = p.2 .0[1] as f32;
        let b = p.2 .0[2] as f32;

        let k = r * 0.3 + g * 0.59 + b * 0.11;
        let character = ((k / 255.0) * (pallete.len() - 1) as f32).round() as usize;
        print!("{}", pallete[character]);
    }
}

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    init_camera();
    // Reset terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    loop {
        // Move to the top left
        print!("{esc}[1;1H", esc = 27 as char);
        take_picture();
    }
}

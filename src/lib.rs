mod utils;

use std::{mem, ptr};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation};

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
    log!("Hello there! Compositor canvas starting/loading");
}

#[wasm_bindgen]
pub fn initialise(element_id: String) -> Result<(), JsValue> {
    log!(
        "Compositor canvas (element_id: String = `{}`) initialisation",
        &element_id
    );

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(&element_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec4 position;
        attribute vec2 textureCoord;

        varying highp vec2 vTextureCoord;

        void main(void) {
            gl_Position = position;
            vTextureCoord = textureCoord;
        }
    "#,
    )?;
    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        varying highp vec2 vTextureCoord;

        uniform sampler2D image;

        void main(void) {
            if(vTextureCoord.x > 0.5) {
            gl_FragColor = texture2D(image, vTextureCoord);
            } else {
            gl_FragColor = vec4(vTextureCoord.x, vTextureCoord.y, 0.0, 1.0);
            }
        }
    "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    let position_location = context.get_attrib_location(&program, "position");
    let texcoord_location = context.get_attrib_location(&program, "textureCoord");
    let texture_location: WebGlUniformLocation =
        context.get_uniform_location(&program, "image").unwrap();

    // Bind shader
    context.use_program(Some(&program));

    // Build model
    let vertices: [f32; 18] = [
        -1.0, -1.0, 0.0, // Bottom left
        1.0, -1.0, 0.0, // Bottem right
        1.0, 1.0, 0.0, // Top right
        -1.0, -1.0, 0.0, // Bottom left
        1.0, 1.0, 0.0, // Top right
        -1.0, 1.0, 0.0, // Top left
    ];

    let vertex_buffer = context
        .create_buffer()
        .ok_or("failed to create vertex buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(
        position_location as u32,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(position_location as u32);

    // Add uvs
    let uvs: [f32; 12] = [
        0.0, 0.0, // Bottom left
        1.0, 0.0, // Bottem right
        1.0, 1.0, // Top right
        0.0, 0.0, // Bottom left
        1.0, 1.0, // Top right
        0.0, 1.0, // Top left
    ];

    let uv_buffer = context
        .create_buffer()
        .ok_or("failed to create uv buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&uv_buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let uv_array = js_sys::Float32Array::view(&uvs);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &uv_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(
        texcoord_location as u32,
        2,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(texcoord_location as u32);

    // Create a texture

    // Add uvs
    //     var textureData = new Uint8Array([128, 128, 0, 255]);
    // var texture = gl.createTexture();
    // gl.bindTexture(gl.TEXTURE_2D, texture);
    // gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 1, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE, textureData);
    // gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
    // gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    // wire up the shader program to the texture data
    // var imageUniform = gl.getUniformLocation(shaderProgram, "image")
    // gl.uniform1i(imageUniform, 0);

    log!("Creating massive block of memory");
    let pixel: [u8; /*640 * 480 **/ 4] = unsafe {
        // Create an uninitialized array.
        let mut array: [u8; /*640 * 480 **/ 4] = mem::uninitialized();

        for (_i, element) in array.iter_mut().enumerate() {
            // Overwrite `element` without running the destructor of the old value.
            // Since Foo does not implement Copy, it is moved.
            ptr::write(element, 255);
        }

        array
    };

    log!("Created massive block of memory");

    let texture = context.create_texture();
    context.bind_texture(WebGlRenderingContext::TEXTURE_2D, texture.as_ref());
    context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        //context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
        WebGlRenderingContext::TEXTURE_2D,
        0,
        WebGlRenderingContext::RGBA as i32,
        1, //640,
        1, //480,
        0,
        WebGlRenderingContext::RGBA,
        WebGlRenderingContext::UNSIGNED_BYTE,
        Some(&pixel),
    );
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MAG_FILTER,
        WebGlRenderingContext::NEAREST as i32,
    );
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::NEAREST as i32,
    );
    // context.generate_mipmap(WebGlRenderingContext::TEXTURE_2D);
    context.uniform1i(Some(texture_location.as_ref()), 0);
    // draw()
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    context.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );

    // Fin
    Ok(())
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

#[wasm_bindgen]
pub fn copy() -> Result<(), JsValue> {
    log!("Compositor copying input");
    Ok(())
}

#[wasm_bindgen]
pub fn render() -> Result<(), JsValue> {
    log!("Compositor rendering composited image to output");
    Ok(())
}

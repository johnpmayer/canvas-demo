
#![recursion_limit="256"]
#[macro_use]
extern crate stdweb;


use stdweb::Value;

use stdweb::web::{
    HtmlElement,

    INode,
};

use stdweb::unstable::TryInto;

mod constants;
mod gl;

use constants::*;
use gl::context::WebGLContext;
use gl::types::ShaderType;

fn main() {
    println!("Hello, world!");

    stdweb::initialize();

    let doc = stdweb::web::document();
    let body = doc.query_selector("body").unwrap();
    let canvas_element: HtmlElement = doc.create_element("canvas").try_into().unwrap();
    body.append_child(&canvas_element);

    let context = WebGLContext::new(&canvas_element);

    // Create shader pipeline

    let vertex_shader = context.create_shader(ShaderType::Vertex, VERTEX_SHADER_SOURCE);
    let fragment_shader = context.create_shader(ShaderType::Fragment, FRAGMENT_SHADER_SOURCE);
    let program = context.create_program(&vertex_shader, &fragment_shader);

    let gl = context.gl;
    
    // Get some data in

    let positions = vec!(0., 0., 0., 0.5, 0.7, 0.);

    let vao: Value = js! {
        var gl = @{&gl};
        var program = @{&program};

        var positionAttributeLocation = gl.getAttribLocation(program, "a_position");
        var positionBuffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

        // three 2d points
        var positions = @{&positions};
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

        var vao = gl.createVertexArray();
        gl.bindVertexArray(vao);
        gl.enableVertexAttribArray(positionAttributeLocation);

        var size = 2;          // 2 components per iteration
        var type = gl.FLOAT;   // the data is 32bit floats
        var normalize = false; // don't normalize the data
        var stride = 0;        // 0 = move forward size * sizeof(type) each iteration to get the next position
        var offset = 0;        // start at the beginning of the buffer
        gl.vertexAttribPointer(positionAttributeLocation, size, type, normalize, stride, offset);

        return vao;
    };

    // FIXME - resize the canvas??? 

    // Draw

    js! {
        var gl = @{&gl};
        var program = @{&program};
        var vao = @{&vao};

        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
        
        gl.clearColor(0, 0, 0, 0);
        gl.clear(gl.COLOR_BUFFER_BIT);

        gl.useProgram(program);

        gl.bindVertexArray(vao);

        var primitiveType = gl.TRIANGLES;
        var offset = 0;
        var count = 3;
        gl.drawArrays(primitiveType, offset, count);
    }

    stdweb::event_loop();
}

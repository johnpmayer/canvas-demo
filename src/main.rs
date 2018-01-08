
#![recursion_limit="256"]
#[macro_use]
extern crate stdweb;

use stdweb::Value;

use stdweb::web::{
    HtmlElement,
    INode,
};

use stdweb::unstable::TryInto;

enum ShaderType {
    Vertex, 
    Fragment,
}

impl ShaderType {
    fn js_value(&self, gl: &Value) -> Value {
        match self {
            &ShaderType::Vertex => js! {
                return @{gl}.VERTEX_SHADER;
            },
            &ShaderType::Fragment => js! {
                return @{gl}.FRAGMENT_SHADER;
            },
        }
    }
}

fn create_shader(gl: &Value, shader_type: ShaderType, source: &str) -> Value {
    let shader_type_js: Value = shader_type.js_value(gl);
    let shader = js! {
        var gl = @{gl};
        var type = @{shader_type_js};
        var source = @{source};

        var shader = gl.createShader(type);
        gl.shaderSource(shader, source);
        gl.compileShader(shader);
        var success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);
        if (success) {
            return shader;
        }
 
        console.log(gl.getShaderInfoLog(shader));
        gl.deleteShader(shader);
    };
    shader
}

fn main() {
    println!("Hello, world!");

    stdweb::initialize();

    let doc = stdweb::web::document();
    let body = doc.query_selector("body").unwrap();
    let canvas: HtmlElement = doc.create_element("canvas").try_into().unwrap();
    body.append_child(&canvas);

    let gl = js! {
        var context = @{canvas}.getContext("webgl2");
        console.log(context);
        return context;
    };

    // Credit: https://webgl2fundamentals.org/webgl/lessons/webgl-fundamentals.html
    // https://github.com/greggman/webgl2-fundamentals/blob/master/LICENSE 
    // BSD 3-clause "New" or "Revised" License
    // f9b5341639255f53eca5da2715a4720e32e8d51a

    let vertex_shader_source = "#version 300 es
 
// an attribute is an input (in) to a vertex shader.
// It will receive data from a buffer
in vec4 a_position;
 
// all shaders have a main function
void main() {
 
  // gl_Position is a special variable a vertex shader
  // is responsible for setting
  gl_Position = a_position;
}";

    let fragment_shader_source = "#version 300 es
 
// fragment shaders don't have a default precision so we need
// to pick one. mediump is a good default. It means medium precision
precision mediump float;
 
// we need to declare an output for the fragment shader
out vec4 outColor;
 
void main() {
  // Just set the output to a constant redish-purple
  outColor = vec4(1, 0, 0.5, 1);
}";

    // Create shader pipeline

    let vertex_shader = create_shader(&gl, ShaderType::Vertex, vertex_shader_source);
    let fragment_shader = create_shader(&gl, ShaderType::Fragment, fragment_shader_source);

    let program = js! {
        var gl = @{&gl};
        var vertexShader = @{&vertex_shader};
        var fragmentShader = @{&fragment_shader};

        var program = gl.createProgram();
        gl.attachShader(program, vertexShader);
        gl.attachShader(program, fragmentShader);
        gl.linkProgram(program);
        var success = gl.getProgramParameter(program, gl.LINK_STATUS);
        if (success) {
            return program;
        }
        
        console.log(gl.getProgramInfoLog(program));
        gl.deleteProgram(program);
    };

    js! {
        console.log(@{&program});
    }

    // Get some data in
    
    let vao: Value = js! {
        var gl = @{&gl};
        var program = @{&program};

        var positionAttributeLocation = gl.getAttribLocation(program, "a_position");
        var positionBuffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

        // three 2d points
        var positions = [
            0, 0,
            0, 0.5,
            0.7, 0,
        ];
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

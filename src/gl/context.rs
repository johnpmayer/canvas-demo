
use stdweb::Value;

use stdweb::web::{
    HtmlElement,
};

use gl::types::ShaderType;

pub struct WebGLContext {
    pub gl: Value, // TODO - make this private
}

impl WebGLContext {
    pub fn new(canvas_element: &HtmlElement) -> WebGLContext {
        let gl = js! {
            var context = @{&canvas_element}.getContext("webgl2");
            console.log(context);
            return context;
        };
        WebGLContext { gl }
    }

    fn shader_value(&self, shader_type: ShaderType) -> Value {
        match shader_type {
            ShaderType::Vertex => js! {
                return @{&self.gl}.VERTEX_SHADER;
            },
            ShaderType::Fragment => js! {
                return @{&self.gl}.FRAGMENT_SHADER;
            },
        }
    }

    pub fn create_shader(&self, shader_type: ShaderType, source: &str) -> Value {
        let shader_type_js: Value = self.shader_value(shader_type);
        let shader = js! {
            var gl = @{&self.gl};
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

    pub fn create_program(&self, vertex_shader: &Value, fragment_shader: &Value) -> Value {
        let program = js! {
            var gl = @{&self.gl};
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
        program
    }
}